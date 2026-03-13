import importlib.metadata
from pathlib import Path

import a2lfile


TEST_A2L = """
ASAP2_VERSION 1 71
A2ML_VERSION 1 31
/begin PROJECT PROJECT_DEMO ""
  /begin MODULE MODULE_DEMO ""
    /begin A2ML
      block "IF_DATA" taggedstruct if_data {
        block "XCP" taggedstruct xcp {
          "PROTOCOL_LAYER" struct {
            uchar;
          };
        };
      };
    /end A2ML
    /begin IF_DATA
      /begin XCP
        PROTOCOL_LAYER 1
      /end XCP
    /end IF_DATA
    /begin MEASUREMENT RPM "Engine speed"
      UBYTE NO_COMPU_METHOD 1 0 0 255
      /begin IF_DATA
        /begin XCP
          PROTOCOL_LAYER 2
        /end XCP
      /end IF_DATA
    /end MEASUREMENT
  /end MODULE
/end PROJECT
""".strip()


TEST_A2L_WITH_CONVERSIONS = """
ASAP2_VERSION 1 71
/begin PROJECT PROJECT_CONVERSIONS ""
  /begin MODULE MODULE_CONVERSIONS ""
    /begin MOD_COMMON ""
      BYTE_ORDER MSB_FIRST
    /end MOD_COMMON
    /begin MOD_PAR ""
      EPK "EPK_TAG"
      ADDR_EPK 0x1234
    /end MOD_PAR
    /begin UNIT temp_unit "" "degC" DERIVED
      UNIT_CONVERSION 1 0
    /end UNIT
    /begin COMPU_METHOD temp_linear "" LINEAR "%8.3" "degC"
      COEFFS_LINEAR 0.1 0.0
      REF_UNIT temp_unit
    /end COMPU_METHOD
    /begin COMPU_TAB temp_numeric "" TAB_NOINTP 2
      0 0
      1 10
      DEFAULT_VALUE_NUMERIC 99
    /end COMPU_TAB
    /begin COMPU_VTAB status_vtab "" TAB_VERB 2
      0 "off"
      1 "on"
      DEFAULT_VALUE "unknown"
    /end COMPU_VTAB
    /begin COMPU_VTAB_RANGE range_vtab "" 1
      0 10 "valid"
      DEFAULT_VALUE "unknown"
    /end COMPU_VTAB_RANGE
    /begin COMPU_METHOD temp_table "" TAB_NOINTP "%3.0" ""
      COMPU_TAB_REF temp_numeric
    /end COMPU_METHOD
    /begin COMPU_METHOD status_table "" TAB_VERB "%3.0" ""
      COMPU_TAB_REF status_vtab
    /end COMPU_METHOD
    /begin COMPU_METHOD range_table "" TAB_VERB "%3.0" ""
      COMPU_TAB_REF range_vtab
    /end COMPU_METHOD
    /begin MEASUREMENT TEMP_LINEAR "Temperature"
      SWORD temp_linear 1 0 -3276.8 3276.7
      /begin ANNOTATION
        ANNOTATION_LABEL "sensor"
        ANNOTATION_ORIGIN "bindings-test"
        /begin ANNOTATION_TEXT
          "line one"
          "line two"
        /end ANNOTATION_TEXT
      /end ANNOTATION
      BIT_MASK 255
      /begin BIT_OPERATION
        LEFT_SHIFT 1
        RIGHT_SHIFT 2
        SIGN_EXTEND
      /end BIT_OPERATION
      BYTE_ORDER MSB_LAST
      DISCRETE
      DISPLAY_IDENTIFIER TempDisplay
      ECU_ADDRESS 0x1000
      ECU_ADDRESS_EXTENSION 1
      ERROR_MASK 255
      FORMAT "%8.3"
      /begin FUNCTION_LIST
        fn_temp
      /end FUNCTION_LIST
      LAYOUT ROW_DIR
      MATRIX_DIM 1 1 1
      MAX_REFRESH 1 100
      MODEL_LINK "Model.Temp"
      PHYS_UNIT "degC"
      READ_WRITE
      REF_MEMORY_SEGMENT seg_temp
      SYMBOL_LINK "temp_symbol" 0
    /end MEASUREMENT
    /begin MEASUREMENT TEMP_TABLE "Temperature table"
      UBYTE temp_table 1 0 0 255
      /begin VIRTUAL
        TEMP_LINEAR
      /end VIRTUAL
    /end MEASUREMENT
    /begin MEASUREMENT STATUS_TABLE "Status table"
      UBYTE status_table 1 0 0 1
    /end MEASUREMENT
    /begin MEASUREMENT RANGE_TABLE "Range table"
      UBYTE range_table 1 0 0 10
    /end MEASUREMENT
  /end MODULE
/end PROJECT
""".strip()


def test_import_exposes_version() -> None:
    assert isinstance(a2lfile.__version__, str)
    assert a2lfile.__version__


def test_distribution_name_is_py_a2lfile() -> None:
    assert importlib.metadata.version("py-a2lfile") == a2lfile.__version__


def test_typing_artifacts_are_present() -> None:
    package_dir = Path(a2lfile.__file__).resolve().parent
    stub_path = package_dir / "_a2lfile.pyi"

    assert stub_path.is_file()
    assert (package_dir / "py.typed").is_file()

    stub_text = stub_path.read_text(encoding="utf-8")
    assert "class A2lFile" in stub_text
    assert "class CompuMethod" in stub_text
    assert "class Unit" in stub_text
    assert "def load(" in stub_text


def test_load_from_string_exposes_modules_measurements_and_if_data() -> None:
    parsed = a2lfile.load_from_string(TEST_A2L)

    assert isinstance(parsed, a2lfile.A2lFile)
    assert len(parsed.modules) == 1

    module = parsed.modules[0]
    assert module.name == "MODULE_DEMO"
    assert module.long_identifier == ""
    assert len(module.measurements) == 1

    measurement = module.measurements[0]
    assert measurement.name == "RPM"
    assert measurement.long_identifier == "Engine speed"
    assert measurement.conversion == "NO_COMPU_METHOD"
    assert measurement.resolution == 1
    assert measurement.lower_limit == 0.0
    assert measurement.upper_limit == 255.0

    module_if_data = module.if_data[0]
    assert module_if_data.valid is True
    assert module_if_data.items.kind == "block"

    tagged_root = module_if_data.items.items[0]
    assert tagged_root.kind == "tagged_struct"
    assert "XCP" in tagged_root.tagged_items

    xcp_item = tagged_root.tagged_items["XCP"][0]
    assert isinstance(xcp_item, a2lfile.GenericIfDataTaggedItem)
    assert xcp_item.tag == "XCP"
    assert xcp_item.is_block is True

    protocol_layer = xcp_item.data.items[0].tagged_items["PROTOCOL_LAYER"][0]
    assert protocol_layer.tag == "PROTOCOL_LAYER"
    assert protocol_layer.data.items[0].kind == "uchar"
    assert protocol_layer.data.items[0].value == 1

    measurement_if_data = measurement.if_data[0]
    protocol_layer = measurement_if_data.items.items[0].tagged_items["XCP"][0]
    protocol_layer = protocol_layer.data.items[0].tagged_items["PROTOCOL_LAYER"][0]
    assert protocol_layer.data.items[0].value == 2


def test_load_reads_from_file(tmp_path: Path) -> None:
    source = tmp_path / "example.a2l"
    source.write_text(TEST_A2L, encoding="utf-8")

    parsed = a2lfile.load(str(source))
    assert parsed.modules[0].measurements[0].name == "RPM"


def test_measurement_dependencies_are_resolved_cleanly() -> None:
    parsed = a2lfile.load_from_string(TEST_A2L_WITH_CONVERSIONS)
    module = parsed.modules[0]

    assert len(module.compu_methods) == 4
    assert len(module.compu_tabs) == 1
    assert len(module.compu_vtabs) == 1
    assert len(module.compu_vtab_ranges) == 1
    assert len(module.units) == 1
    assert module.mod_common_byte_order == "MSB_FIRST"
    assert module.mod_par_epk == "EPK_TAG"
    assert module.mod_par_addr_epk == [0x1234]

    temp_linear = module.get_measurement("TEMP_LINEAR")
    assert isinstance(temp_linear, a2lfile.Measurement)
    assert temp_linear.compu_method is not None
    assert temp_linear.compu_method.name == "temp_linear"
    assert temp_linear.compu_method.conversion_type == "LINEAR"
    assert temp_linear.compu_method.coeffs_linear.a == 0.1
    assert temp_linear.compu_method.coeffs_linear.b == 0.0
    assert temp_linear.compu_method.referenced_unit.name == "temp_unit"
    assert temp_linear.compu_method.referenced_unit.display == "degC"
    assert temp_linear.compu_method.referenced_unit.unit_conversion.gradient == 1.0
    assert temp_linear.compu_method.referenced_unit.unit_conversion.offset == 0.0

    assert temp_linear.annotation[0].label == "sensor"
    assert temp_linear.annotation[0].origin == "bindings-test"
    assert temp_linear.annotation[0].text_lines == ["line one", "line two"]
    assert temp_linear.bit_mask == 255
    assert temp_linear.bit_operation.left_shift == 1
    assert temp_linear.bit_operation.right_shift == 2
    assert temp_linear.bit_operation.sign_extend is True
    assert temp_linear.byte_order == "MSB_LAST"
    assert temp_linear.discrete is True
    assert temp_linear.display_identifier == "TempDisplay"
    assert temp_linear.ecu_address == 0x1000
    assert temp_linear.ecu_address_extension == 1
    assert temp_linear.error_mask == 255
    assert temp_linear.format == "%8.3"
    assert temp_linear.function_list == ["fn_temp"]
    assert temp_linear.layout == "ROW_DIR"
    assert temp_linear.matrix_dim == [1, 1, 1]
    assert temp_linear.max_refresh.scaling_unit == 1
    assert temp_linear.max_refresh.rate == 100
    assert temp_linear.model_link == "Model.Temp"
    assert temp_linear.phys_unit == "degC"
    assert temp_linear.read_write is True
    assert temp_linear.ref_memory_segment == "seg_temp"
    assert temp_linear.symbol_link.symbol_name == "temp_symbol"
    assert temp_linear.symbol_link.offset == 0

    temp_table = module.get_measurement("TEMP_TABLE")
    table = temp_table.compu_method.referenced_table
    assert isinstance(table, a2lfile.CompuTab)
    assert table.entries[1].in_val == 1.0
    assert table.entries[1].out_val == 10.0
    assert table.default_value_numeric == 99.0
    assert temp_table.virtual == ["TEMP_LINEAR"]

    status_table = module.get_measurement("STATUS_TABLE").compu_method.referenced_table
    assert isinstance(status_table, a2lfile.CompuVtab)
    assert status_table.entries[1].out_val == "on"
    assert status_table.default_value == "unknown"

    range_table = module.get_measurement("RANGE_TABLE").compu_method.referenced_table
    assert isinstance(range_table, a2lfile.CompuVtabRange)
    assert range_table.entries[0].in_val_min == 0.0
    assert range_table.entries[0].in_val_max == 10.0
    assert range_table.entries[0].out_val == "valid"

    assert isinstance(module.get_compu_method("temp_linear"), a2lfile.CompuMethod)
    assert isinstance(module.get_unit("temp_unit"), a2lfile.Unit)
    assert isinstance(module.get_compu_tab("temp_numeric"), a2lfile.CompuTab)
