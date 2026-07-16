#![allow(unsafe_op_in_unsafe_fn)]

use std::collections::HashMap;
use std::sync::Arc;

use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyModule as PyModuleType;
use pyo3::wrap_pyfunction;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::{
    gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pyfunction, gen_stub_pymethods,
};

use a2lfile_core::{
    A2lFile as RustA2lFile, A2lObjectName, AddrType as RustAddrType, Annotation as RustAnnotation,
    BitOperation as RustBitOperation, ByteOrderEnum as RustByteOrderEnum, Coeffs as RustCoeffs,
    CoeffsLinear as RustCoeffsLinear, CompuMethod as RustCompuMethod, CompuTab as RustCompuTab,
    CompuVtab as RustCompuVtab, CompuVtabRange as RustCompuVtabRange,
    ConversionType as RustConversionType, DataType as RustDataType, Formula as RustFormula,
    GenericIfData, GenericIfDataTaggedItem, IfData as RustIfData, IndexMode as RustIndexMode,
    MaxRefresh as RustMaxRefresh, Measurement as RustMeasurement, Module as RustModule,
    SiExponents as RustSiExponents, SymbolLink as RustSymbolLink, TabEntryStruct as RustTabEntry,
    Unit as RustUnit, UnitConversion as RustUnitConversion, UnitType as RustUnitType,
    ValuePairsStruct as RustValuePair, ValueTriplesStruct as RustValueTriple,
};

pyo3_stub_gen::module_variable!("a2lfile._a2lfile", "__version__", String);

fn map_a2l_error(err: a2lfile_core::A2lError) -> PyErr {
    PyException::new_err(err.to_string())
}

// Mirror the upstream unit enums so Python callers can use typed A2L values.
#[gen_stub_pyclass_enum]
#[pyclass(
    name = "AddrType",
    module = "a2lfile._a2lfile",
    eq,
    frozen,
    hash,
    skip_from_py_object
)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PyAddrType {
    Pbyte,
    Pword,
    Plong,
    Plonglong,
    Direct,
}

impl From<&RustAddrType> for PyAddrType {
    fn from(value: &RustAddrType) -> Self {
        match value {
            RustAddrType::Pbyte => Self::Pbyte,
            RustAddrType::Pword => Self::Pword,
            RustAddrType::Plong => Self::Plong,
            RustAddrType::Plonglong => Self::Plonglong,
            RustAddrType::Direct => Self::Direct,
        }
    }
}

#[gen_stub_pyclass_enum]
#[pyclass(
    name = "ByteOrderEnum",
    module = "a2lfile._a2lfile",
    eq,
    frozen,
    hash,
    skip_from_py_object
)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PyByteOrderEnum {
    LittleEndian,
    BigEndian,
    MsbLast,
    MsbFirst,
    MsbFirstMswLast,
    MsbLastMswFirst,
}

impl From<&RustByteOrderEnum> for PyByteOrderEnum {
    fn from(value: &RustByteOrderEnum) -> Self {
        match value {
            RustByteOrderEnum::LittleEndian => Self::LittleEndian,
            RustByteOrderEnum::BigEndian => Self::BigEndian,
            RustByteOrderEnum::MsbLast => Self::MsbLast,
            RustByteOrderEnum::MsbFirst => Self::MsbFirst,
            RustByteOrderEnum::MsbFirstMswLast => Self::MsbFirstMswLast,
            RustByteOrderEnum::MsbLastMswFirst => Self::MsbLastMswFirst,
        }
    }
}

#[gen_stub_pyclass_enum]
#[pyclass(
    name = "ConversionType",
    module = "a2lfile._a2lfile",
    eq,
    frozen,
    hash,
    skip_from_py_object
)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PyConversionType {
    Identical,
    Form,
    Linear,
    RatFunc,
    TabIntp,
    TabNointp,
    TabVerb,
}

impl From<&RustConversionType> for PyConversionType {
    fn from(value: &RustConversionType) -> Self {
        match value {
            RustConversionType::Identical => Self::Identical,
            RustConversionType::Form => Self::Form,
            RustConversionType::Linear => Self::Linear,
            RustConversionType::RatFunc => Self::RatFunc,
            RustConversionType::TabIntp => Self::TabIntp,
            RustConversionType::TabNointp => Self::TabNointp,
            RustConversionType::TabVerb => Self::TabVerb,
        }
    }
}

#[gen_stub_pyclass_enum]
#[pyclass(
    name = "DataType",
    module = "a2lfile._a2lfile",
    eq,
    frozen,
    hash,
    skip_from_py_object
)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PyDataType {
    Ubyte,
    Sbyte,
    Uword,
    Sword,
    Ulong,
    Slong,
    AUint64,
    AInt64,
    Float16Ieee,
    Float32Ieee,
    Float64Ieee,
}

impl From<&RustDataType> for PyDataType {
    fn from(value: &RustDataType) -> Self {
        match value {
            RustDataType::Ubyte => Self::Ubyte,
            RustDataType::Sbyte => Self::Sbyte,
            RustDataType::Uword => Self::Uword,
            RustDataType::Sword => Self::Sword,
            RustDataType::Ulong => Self::Ulong,
            RustDataType::Slong => Self::Slong,
            RustDataType::AUint64 => Self::AUint64,
            RustDataType::AInt64 => Self::AInt64,
            RustDataType::Float16Ieee => Self::Float16Ieee,
            RustDataType::Float32Ieee => Self::Float32Ieee,
            RustDataType::Float64Ieee => Self::Float64Ieee,
        }
    }
}

#[gen_stub_pyclass_enum]
#[pyclass(
    name = "IndexMode",
    module = "a2lfile._a2lfile",
    eq,
    frozen,
    hash,
    skip_from_py_object
)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PyIndexMode {
    AlternateCurves,
    AlternateWithX,
    AlternateWithY,
    ColumnDir,
    RowDir,
}

impl From<&RustIndexMode> for PyIndexMode {
    fn from(value: &RustIndexMode) -> Self {
        match value {
            RustIndexMode::AlternateCurves => Self::AlternateCurves,
            RustIndexMode::AlternateWithX => Self::AlternateWithX,
            RustIndexMode::AlternateWithY => Self::AlternateWithY,
            RustIndexMode::ColumnDir => Self::ColumnDir,
            RustIndexMode::RowDir => Self::RowDir,
        }
    }
}

#[gen_stub_pyclass_enum]
#[pyclass(
    name = "UnitType",
    module = "a2lfile._a2lfile",
    eq,
    frozen,
    hash,
    skip_from_py_object
)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PyUnitType {
    Derived,
    ExtendedSi,
}

impl From<&RustUnitType> for PyUnitType {
    fn from(value: &RustUnitType) -> Self {
        match value {
            RustUnitType::Derived => Self::Derived,
            RustUnitType::ExtendedSi => Self::ExtendedSi,
        }
    }
}

#[derive(Clone)]
struct ModuleLookupContext {
    a2l: Arc<RustA2lFile>,
    module_index: usize,
}

impl ModuleLookupContext {
    fn new(a2l: Arc<RustA2lFile>, module_index: usize) -> Self {
        Self { a2l, module_index }
    }

    fn module(&self) -> &RustModule {
        // Resolve module data on demand so nested views can share the parsed file.
        self.a2l
            .project
            .module
            .iter()
            .nth(self.module_index)
            .expect("module index should stay valid for the lifetime of the parsed file")
    }

    fn resolve_measurement(&self, name: &str) -> Option<&RustMeasurement> {
        self.module().measurement.get(name)
    }

    fn resolve_compu_method(&self, name: &str) -> Option<RustCompuMethod> {
        if name == "NO_COMPU_METHOD" {
            return None;
        }
        self.module().compu_method.get(name).cloned()
    }

    fn resolve_unit(&self, name: &str) -> Option<RustUnit> {
        self.module().unit.get(name).cloned()
    }

    fn resolve_table(&self, name: &str) -> Option<ResolvedCompuTable> {
        // Resolve referenced conversion tables lazily from the shared module data.
        if let Some(compu_tab) = self.module().compu_tab.get(name) {
            return Some(ResolvedCompuTable::CompuTab(compu_tab.clone()));
        }
        if let Some(compu_vtab) = self.module().compu_vtab.get(name) {
            return Some(ResolvedCompuTable::CompuVtab(compu_vtab.clone()));
        }
        if let Some(compu_vtab_range) = self.module().compu_vtab_range.get(name) {
            return Some(ResolvedCompuTable::CompuVtabRange(compu_vtab_range.clone()));
        }
        None
    }
}

enum ResolvedCompuTable {
    CompuTab(RustCompuTab),
    CompuVtab(RustCompuVtab),
    CompuVtabRange(RustCompuVtabRange),
}

fn wrap_resolved_table(py: Python<'_>, table: ResolvedCompuTable) -> PyResult<Py<PyAny>> {
    match table {
        ResolvedCompuTable::CompuTab(inner) => {
            Ok(Py::new(py, PyCompuTabView::new(inner))?.into_any())
        }
        ResolvedCompuTable::CompuVtab(inner) => {
            Ok(Py::new(py, PyCompuVtabView::new(inner))?.into_any())
        }
        ResolvedCompuTable::CompuVtabRange(inner) => {
            Ok(Py::new(py, PyCompuVtabRangeView::new(inner))?.into_any())
        }
    }
}

fn wrap_resolved_table_by_name(
    py: Python<'_>,
    lookup: &ModuleLookupContext,
    name: &str,
) -> PyResult<Option<Py<PyAny>>> {
    lookup
        .resolve_table(name)
        .map(|table| wrap_resolved_table(py, table))
        .transpose()
}

#[gen_stub_pyclass]
#[pyclass(name = "A2lFile", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyA2lFile {
    inner: Arc<RustA2lFile>,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyA2lFile {
    #[gen_stub(override_return_type(type_repr = "list[Module]"))]
    #[getter]
    fn modules(&self) -> Vec<PyModuleView> {
        // Expose lightweight module views so Python does not duplicate the full parse tree.
        self.inner
            .project
            .module
            .iter()
            .enumerate()
            .map(|(module_index, _)| PyModuleView::new(Arc::clone(&self.inner), module_index))
            .collect()
    }

    #[gen_stub(skip)]
    fn __repr__(&self) -> String {
        format!("A2lFile(modules={})", self.inner.project.module.len())
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Module", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyModuleView {
    a2l: Arc<RustA2lFile>,
    module_index: usize,
}

impl PyModuleView {
    fn new(a2l: Arc<RustA2lFile>, module_index: usize) -> Self {
        Self { a2l, module_index }
    }

    fn inner(&self) -> &RustModule {
        // Resolve the module on demand from the shared parsed file instead of storing a clone.
        self.a2l
            .project
            .module
            .iter()
            .nth(self.module_index)
            .expect("module index should stay valid for the lifetime of the parsed file")
    }

    fn lookup(&self) -> Arc<ModuleLookupContext> {
        // Nested views share a lightweight module locator instead of cloned lookup tables.
        Arc::new(ModuleLookupContext::new(
            Arc::clone(&self.a2l),
            self.module_index,
        ))
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyModuleView {
    #[getter]
    fn name(&self) -> String {
        self.inner().get_name().to_string()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner().long_identifier.clone()
    }

    #[gen_stub(override_return_type(type_repr = "list[Measurement]"))]
    #[getter]
    fn measurements(&self) -> Vec<PyMeasurementView> {
        // Reuse one lookup context for the full returned measurement list.
        let lookup = self.lookup();
        self.inner()
            .measurement
            .iter()
            .map(|inner| PyMeasurementView::new(inner.get_name().to_string(), Arc::clone(&lookup)))
            .collect()
    }

    #[gen_stub(override_return_type(type_repr = "list[CompuMethod]"))]
    #[getter]
    fn compu_methods(&self) -> Vec<PyCompuMethodView> {
        // Reuse one lookup context for the full returned conversion list.
        let lookup = self.lookup();
        self.inner()
            .compu_method
            .iter()
            .cloned()
            .map(|inner| PyCompuMethodView::new(inner, Arc::clone(&lookup)))
            .collect()
    }

    #[gen_stub(override_return_type(type_repr = "list[CompuTab]"))]
    #[getter]
    fn compu_tabs(&self) -> Vec<PyCompuTabView> {
        self.inner()
            .compu_tab
            .iter()
            .cloned()
            .map(PyCompuTabView::new)
            .collect()
    }

    #[gen_stub(override_return_type(type_repr = "list[CompuVtab]"))]
    #[getter]
    fn compu_vtabs(&self) -> Vec<PyCompuVtabView> {
        self.inner()
            .compu_vtab
            .iter()
            .cloned()
            .map(PyCompuVtabView::new)
            .collect()
    }

    #[gen_stub(override_return_type(type_repr = "list[CompuVtabRange]"))]
    #[getter]
    fn compu_vtab_ranges(&self) -> Vec<PyCompuVtabRangeView> {
        self.inner()
            .compu_vtab_range
            .iter()
            .cloned()
            .map(PyCompuVtabRangeView::new)
            .collect()
    }

    #[gen_stub(override_return_type(type_repr = "list[Unit]"))]
    #[getter]
    fn units(&self) -> Vec<PyUnitView> {
        // Reuse one lookup context for the full returned unit list.
        let lookup = self.lookup();
        self.inner()
            .unit
            .iter()
            .cloned()
            .map(|inner| PyUnitView::new(inner, Arc::clone(&lookup)))
            .collect()
    }

    #[gen_stub(override_return_type(type_repr = "list[IfData]"))]
    #[getter]
    fn if_data(&self) -> Vec<PyIfDataView> {
        self.inner()
            .if_data
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[gen_stub(override_return_type(type_repr = "ByteOrderEnum | None"))]
    #[getter]
    fn mod_common_byte_order(&self) -> Option<PyByteOrderEnum> {
        self.inner()
            .mod_common
            .as_ref()
            .and_then(|value| value.byte_order.as_ref())
            .map(|value| PyByteOrderEnum::from(&value.byte_order))
    }

    #[getter]
    fn mod_par_epk(&self) -> Option<String> {
        self.inner()
            .mod_par
            .as_ref()
            .and_then(|value| value.epk.as_ref())
            .map(|value| value.identifier.clone())
    }

    #[gen_stub(override_return_type(type_repr = "list[int]"))]
    #[getter]
    fn mod_par_addr_epk(&self) -> Vec<u32> {
        self.inner()
            .mod_par
            .as_ref()
            .map(|value| value.addr_epk.iter().map(|addr| addr.address).collect())
            .unwrap_or_default()
    }

    #[gen_stub(override_return_type(type_repr = "Measurement | None"))]
    fn get_measurement(&self, name: &str) -> Option<PyMeasurementView> {
        // Create lookup state only when returning a nested measurement view.
        let lookup = self.lookup();
        self.inner()
            .measurement
            .get(name)
            .map(|inner| PyMeasurementView::new(inner.get_name().to_string(), lookup))
    }

    #[gen_stub(override_return_type(type_repr = "CompuMethod | None"))]
    fn get_compu_method(&self, name: &str) -> Option<PyCompuMethodView> {
        // Resolve the conversion from shared module data, then attach the same lookup cache.
        let lookup = self.lookup();
        lookup
            .resolve_compu_method(name)
            .map(|inner| PyCompuMethodView::new(inner, lookup))
    }

    #[gen_stub(override_return_type(type_repr = "Unit | None"))]
    fn get_unit(&self, name: &str) -> Option<PyUnitView> {
        // Resolve the unit from shared module data, then attach the same lookup cache.
        let lookup = self.lookup();
        lookup
            .resolve_unit(name)
            .map(|inner| PyUnitView::new(inner, lookup))
    }

    #[gen_stub(override_return_type(type_repr = "CompuTab | CompuVtab | CompuVtabRange | None"))]
    fn get_compu_tab(&self, py: Python<'_>, name: &str) -> PyResult<Option<Py<PyAny>>> {
        let lookup = self.lookup();
        wrap_resolved_table_by_name(py, &lookup, name)
    }

    #[gen_stub(skip)]
    fn __repr__(&self) -> String {
        format!(
            "Module(name={:?}, measurements={})",
            self.inner().get_name(),
            self.inner().measurement.len()
        )
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Measurement", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyMeasurementView {
    name: String,
    lookup: Arc<ModuleLookupContext>,
}

impl PyMeasurementView {
    fn new(name: String, lookup: Arc<ModuleLookupContext>) -> Self {
        Self { name, lookup }
    }

    fn inner(&self) -> &RustMeasurement {
        // Resolve measurement data by name so storing the Python view stays lightweight.
        self.lookup
            .resolve_measurement(&self.name)
            .expect("measurement name should stay valid for the lifetime of the parsed file")
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyMeasurementView {
    #[getter]
    fn name(&self) -> String {
        self.name.clone()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner().long_identifier.clone()
    }

    #[gen_stub(override_return_type(type_repr = "DataType"))]
    #[getter]
    fn datatype(&self) -> PyDataType {
        PyDataType::from(&self.inner().datatype)
    }

    #[getter]
    fn conversion(&self) -> String {
        self.inner().conversion.clone()
    }

    #[gen_stub(override_return_type(type_repr = "CompuMethod | None"))]
    #[getter]
    fn compu_method(&self) -> Option<PyCompuMethodView> {
        self.lookup
            .resolve_compu_method(&self.inner().conversion)
            .map(|inner| PyCompuMethodView::new(inner, Arc::clone(&self.lookup)))
    }

    #[getter]
    fn resolution(&self) -> u16 {
        self.inner().resolution
    }

    #[getter]
    fn accuracy(&self) -> f64 {
        self.inner().accuracy
    }

    #[getter]
    fn lower_limit(&self) -> f64 {
        self.inner().lower_limit
    }

    #[getter]
    fn upper_limit(&self) -> f64 {
        self.inner().upper_limit
    }

    #[gen_stub(override_return_type(type_repr = "AddrType | None"))]
    #[getter]
    fn address_type(&self) -> Option<PyAddrType> {
        self.inner()
            .address_type
            .as_ref()
            .map(|value| PyAddrType::from(&value.address_type))
    }

    #[gen_stub(override_return_type(type_repr = "list[Annotation]"))]
    #[getter]
    fn annotation(&self) -> Vec<PyAnnotationView> {
        self.inner()
            .annotation
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn array_size(&self) -> Option<u16> {
        self.inner().array_size.as_ref().map(|value| value.number)
    }

    #[getter]
    fn bit_mask(&self) -> Option<u64> {
        self.inner().bit_mask.as_ref().map(|value| value.mask)
    }

    #[gen_stub(override_return_type(type_repr = "BitOperation | None"))]
    #[getter]
    fn bit_operation(&self) -> Option<PyBitOperationView> {
        self.inner().bit_operation.clone().map(Into::into)
    }

    #[gen_stub(override_return_type(type_repr = "ByteOrderEnum | None"))]
    #[getter]
    fn byte_order(&self) -> Option<PyByteOrderEnum> {
        self.inner()
            .byte_order
            .as_ref()
            .map(|value| PyByteOrderEnum::from(&value.byte_order))
    }

    #[getter]
    fn discrete(&self) -> bool {
        self.inner().discrete.is_some()
    }

    #[getter]
    fn display_identifier(&self) -> Option<String> {
        self.inner()
            .display_identifier
            .as_ref()
            .map(|value| value.display_name.clone())
    }

    #[getter]
    fn ecu_address(&self) -> Option<u32> {
        self.inner().ecu_address.as_ref().map(|value| value.address)
    }

    #[getter]
    fn ecu_address_extension(&self) -> Option<i16> {
        self.inner()
            .ecu_address_extension
            .as_ref()
            .map(|value| value.extension)
    }

    #[getter]
    fn error_mask(&self) -> Option<u64> {
        self.inner().error_mask.as_ref().map(|value| value.mask)
    }

    #[getter]
    fn format(&self) -> Option<String> {
        self.inner()
            .format
            .as_ref()
            .map(|value| value.format_string.clone())
    }

    #[getter]
    fn function_list(&self) -> Option<Vec<String>> {
        self.inner()
            .function_list
            .as_ref()
            .map(|value| value.name_list.clone())
    }

    #[gen_stub(override_return_type(type_repr = "list[IfData]"))]
    #[getter]
    fn if_data(&self) -> Vec<PyIfDataView> {
        self.inner()
            .if_data
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[gen_stub(override_return_type(type_repr = "IndexMode | None"))]
    #[getter]
    fn layout(&self) -> Option<PyIndexMode> {
        self.inner()
            .layout
            .as_ref()
            .map(|value| PyIndexMode::from(&value.index_mode))
    }

    #[getter]
    fn matrix_dim(&self) -> Option<Vec<u16>> {
        self.inner()
            .matrix_dim
            .as_ref()
            .map(|value| value.dim_list.clone())
    }

    #[gen_stub(override_return_type(type_repr = "MaxRefresh | None"))]
    #[getter]
    fn max_refresh(&self) -> Option<PyMaxRefreshView> {
        self.inner().max_refresh.clone().map(Into::into)
    }

    #[getter]
    fn model_link(&self) -> Option<String> {
        self.inner()
            .model_link
            .as_ref()
            .map(|value| value.model_link.clone())
    }

    #[getter]
    fn phys_unit(&self) -> Option<String> {
        self.inner()
            .phys_unit
            .as_ref()
            .map(|value| value.unit.clone())
    }

    #[getter]
    fn read_write(&self) -> bool {
        self.inner().read_write.is_some()
    }

    #[getter]
    fn ref_memory_segment(&self) -> Option<String> {
        self.inner()
            .ref_memory_segment
            .as_ref()
            .map(|value| value.name.clone())
    }

    #[gen_stub(override_return_type(type_repr = "SymbolLink | None"))]
    #[getter]
    fn symbol_link(&self) -> Option<PySymbolLinkView> {
        self.inner().symbol_link.clone().map(Into::into)
    }

    #[getter]
    fn r#virtual(&self) -> Option<Vec<String>> {
        self.inner()
            .var_virtual
            .as_ref()
            .map(|value| value.measuring_channel_list.clone())
    }

    #[gen_stub(skip)]
    fn __repr__(&self) -> String {
        format!(
            "Measurement(name={:?}, datatype={:?})",
            self.name,
            format!("{:?}", self.inner().datatype)
        )
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Annotation", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyAnnotationView {
    inner: RustAnnotation,
}

impl From<RustAnnotation> for PyAnnotationView {
    fn from(inner: RustAnnotation) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyAnnotationView {
    #[getter]
    fn label(&self) -> Option<String> {
        self.inner
            .annotation_label
            .as_ref()
            .map(|value| value.label.clone())
    }

    #[getter]
    fn origin(&self) -> Option<String> {
        self.inner
            .annotation_origin
            .as_ref()
            .map(|value| value.origin.clone())
    }

    #[getter]
    fn text_lines(&self) -> Option<Vec<String>> {
        self.inner
            .annotation_text
            .as_ref()
            .map(|value| value.annotation_text_list.clone())
    }
}

#[gen_stub_pyclass]
#[pyclass(
    name = "BitOperation",
    module = "a2lfile._a2lfile",
    skip_from_py_object
)]
#[derive(Clone)]
struct PyBitOperationView {
    inner: RustBitOperation,
}

impl From<RustBitOperation> for PyBitOperationView {
    fn from(inner: RustBitOperation) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyBitOperationView {
    #[getter]
    fn left_shift(&self) -> Option<u32> {
        self.inner.left_shift.as_ref().map(|value| value.bitcount)
    }

    #[getter]
    fn right_shift(&self) -> Option<u32> {
        self.inner.right_shift.as_ref().map(|value| value.bitcount)
    }

    #[getter]
    fn sign_extend(&self) -> bool {
        self.inner.sign_extend.is_some()
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "MaxRefresh", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyMaxRefreshView {
    inner: RustMaxRefresh,
}

impl From<RustMaxRefresh> for PyMaxRefreshView {
    fn from(inner: RustMaxRefresh) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyMaxRefreshView {
    #[getter]
    fn scaling_unit(&self) -> u16 {
        self.inner.scaling_unit
    }

    #[getter]
    fn rate(&self) -> u32 {
        self.inner.rate
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "SymbolLink", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PySymbolLinkView {
    inner: RustSymbolLink,
}

impl From<RustSymbolLink> for PySymbolLinkView {
    fn from(inner: RustSymbolLink) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySymbolLinkView {
    #[getter]
    fn symbol_name(&self) -> String {
        self.inner.symbol_name.clone()
    }

    #[getter]
    fn offset(&self) -> i32 {
        self.inner.offset
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Coeffs", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyCoeffsView {
    inner: RustCoeffs,
}

impl From<RustCoeffs> for PyCoeffsView {
    fn from(inner: RustCoeffs) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCoeffsView {
    #[getter]
    fn a(&self) -> f64 {
        self.inner.a
    }

    #[getter]
    fn b(&self) -> f64 {
        self.inner.b
    }

    #[getter]
    fn c(&self) -> f64 {
        self.inner.c
    }

    #[getter]
    fn d(&self) -> f64 {
        self.inner.d
    }

    #[getter]
    fn e(&self) -> f64 {
        self.inner.e
    }

    #[getter]
    fn f(&self) -> f64 {
        self.inner.f
    }
}

#[gen_stub_pyclass]
#[pyclass(
    name = "CoeffsLinear",
    module = "a2lfile._a2lfile",
    skip_from_py_object
)]
#[derive(Clone)]
struct PyCoeffsLinearView {
    inner: RustCoeffsLinear,
}

impl From<RustCoeffsLinear> for PyCoeffsLinearView {
    fn from(inner: RustCoeffsLinear) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCoeffsLinearView {
    #[getter]
    fn a(&self) -> f64 {
        self.inner.a
    }

    #[getter]
    fn b(&self) -> f64 {
        self.inner.b
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Formula", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyFormulaView {
    inner: RustFormula,
}

impl From<RustFormula> for PyFormulaView {
    fn from(inner: RustFormula) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyFormulaView {
    #[getter]
    fn fx(&self) -> String {
        self.inner.fx.clone()
    }

    #[getter]
    fn formula_inv(&self) -> Option<String> {
        self.inner
            .formula_inv
            .as_ref()
            .map(|value| value.gx.clone())
    }
}

#[gen_stub_pyclass]
#[pyclass(
    name = "UnitConversion",
    module = "a2lfile._a2lfile",
    skip_from_py_object
)]
#[derive(Clone)]
struct PyUnitConversionView {
    inner: RustUnitConversion,
}

impl From<RustUnitConversion> for PyUnitConversionView {
    fn from(inner: RustUnitConversion) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyUnitConversionView {
    #[getter]
    fn gradient(&self) -> f64 {
        self.inner.gradient
    }

    #[getter]
    fn offset(&self) -> f64 {
        self.inner.offset
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "SiExponents", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PySiExponentsView {
    inner: RustSiExponents,
}

impl From<RustSiExponents> for PySiExponentsView {
    fn from(inner: RustSiExponents) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySiExponentsView {
    #[getter]
    fn length(&self) -> i16 {
        self.inner.length
    }

    #[getter]
    fn mass(&self) -> i16 {
        self.inner.mass
    }

    #[getter]
    fn time(&self) -> i16 {
        self.inner.time
    }

    #[getter]
    fn electric_current(&self) -> i16 {
        self.inner.electric_current
    }

    #[getter]
    fn temperature(&self) -> i16 {
        self.inner.temperature
    }

    #[getter]
    fn amount_of_substance(&self) -> i16 {
        self.inner.amount_of_substance
    }

    #[getter]
    fn luminous_intensity(&self) -> i16 {
        self.inner.luminous_intensity
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Unit", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyUnitView {
    inner: RustUnit,
    lookup: Arc<ModuleLookupContext>,
}

impl PyUnitView {
    fn new(inner: RustUnit, lookup: Arc<ModuleLookupContext>) -> Self {
        Self { inner, lookup }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyUnitView {
    #[getter]
    fn name(&self) -> String {
        self.inner.get_name().to_string()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner.long_identifier.clone()
    }

    #[getter]
    fn display(&self) -> String {
        self.inner.display.clone()
    }

    #[gen_stub(override_return_type(type_repr = "UnitType"))]
    #[getter]
    fn unit_type(&self) -> PyUnitType {
        PyUnitType::from(&self.inner.unit_type)
    }

    #[getter]
    fn ref_unit(&self) -> Option<String> {
        self.inner.ref_unit.as_ref().map(|value| value.unit.clone())
    }

    #[gen_stub(override_return_type(type_repr = "Unit | None"))]
    #[getter]
    fn referenced_unit(&self) -> Option<PyUnitView> {
        self.inner
            .ref_unit
            .as_ref()
            .and_then(|value| self.lookup.resolve_unit(&value.unit))
            .map(|inner| PyUnitView::new(inner, Arc::clone(&self.lookup)))
    }

    #[gen_stub(override_return_type(type_repr = "SiExponents | None"))]
    #[getter]
    fn si_exponents(&self) -> Option<PySiExponentsView> {
        self.inner.si_exponents.clone().map(Into::into)
    }

    #[gen_stub(override_return_type(type_repr = "UnitConversion | None"))]
    #[getter]
    fn unit_conversion(&self) -> Option<PyUnitConversionView> {
        self.inner.unit_conversion.clone().map(Into::into)
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "CompuMethod", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyCompuMethodView {
    inner: RustCompuMethod,
    lookup: Arc<ModuleLookupContext>,
}

impl PyCompuMethodView {
    fn new(inner: RustCompuMethod, lookup: Arc<ModuleLookupContext>) -> Self {
        Self { inner, lookup }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCompuMethodView {
    #[getter]
    fn name(&self) -> String {
        self.inner.get_name().to_string()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner.long_identifier.clone()
    }

    #[gen_stub(override_return_type(type_repr = "ConversionType"))]
    #[getter]
    fn conversion_type(&self) -> PyConversionType {
        PyConversionType::from(&self.inner.conversion_type)
    }

    #[getter]
    fn format(&self) -> String {
        self.inner.format.clone()
    }

    #[getter]
    fn unit(&self) -> String {
        self.inner.unit.clone()
    }

    #[gen_stub(override_return_type(type_repr = "Coeffs | None"))]
    #[getter]
    fn coeffs(&self) -> Option<PyCoeffsView> {
        self.inner.coeffs.clone().map(Into::into)
    }

    #[gen_stub(override_return_type(type_repr = "CoeffsLinear | None"))]
    #[getter]
    fn coeffs_linear(&self) -> Option<PyCoeffsLinearView> {
        self.inner.coeffs_linear.clone().map(Into::into)
    }

    #[getter]
    fn compu_tab_ref(&self) -> Option<String> {
        self.inner
            .compu_tab_ref
            .as_ref()
            .map(|value| value.conversion_table.clone())
    }

    #[gen_stub(override_return_type(type_repr = "Formula | None"))]
    #[getter]
    fn formula(&self) -> Option<PyFormulaView> {
        self.inner.formula.clone().map(Into::into)
    }

    #[getter]
    fn ref_unit(&self) -> Option<String> {
        self.inner.ref_unit.as_ref().map(|value| value.unit.clone())
    }

    #[getter]
    fn status_string_ref(&self) -> Option<String> {
        self.inner
            .status_string_ref
            .as_ref()
            .map(|value| value.conversion_table.clone())
    }

    #[gen_stub(override_return_type(type_repr = "Unit | None"))]
    #[getter]
    fn referenced_unit(&self) -> Option<PyUnitView> {
        self.inner
            .ref_unit
            .as_ref()
            .and_then(|value| self.lookup.resolve_unit(&value.unit))
            .map(|inner| PyUnitView::new(inner, Arc::clone(&self.lookup)))
    }

    #[gen_stub(override_return_type(type_repr = "CompuTab | CompuVtab | CompuVtabRange | None"))]
    #[getter]
    fn referenced_table(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        if let Some(compu_tab_ref) = &self.inner.compu_tab_ref {
            return wrap_resolved_table_by_name(py, &self.lookup, &compu_tab_ref.conversion_table);
        }
        Ok(None)
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "TabEntry", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyTabEntryView {
    inner: RustTabEntry,
}

impl From<RustTabEntry> for PyTabEntryView {
    fn from(inner: RustTabEntry) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyTabEntryView {
    #[getter]
    fn in_val(&self) -> f64 {
        self.inner.in_val
    }

    #[getter]
    fn out_val(&self) -> f64 {
        self.inner.out_val
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "ValuePair", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyValuePairView {
    inner: RustValuePair,
}

impl From<RustValuePair> for PyValuePairView {
    fn from(inner: RustValuePair) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyValuePairView {
    #[getter]
    fn in_val(&self) -> f64 {
        self.inner.in_val
    }

    #[getter]
    fn out_val(&self) -> String {
        self.inner.out_val.clone()
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "ValueTriple", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyValueTripleView {
    inner: RustValueTriple,
}

impl From<RustValueTriple> for PyValueTripleView {
    fn from(inner: RustValueTriple) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyValueTripleView {
    #[getter]
    fn in_val_min(&self) -> f64 {
        self.inner.in_val_min
    }

    #[getter]
    fn in_val_max(&self) -> f64 {
        self.inner.in_val_max
    }

    #[getter]
    fn out_val(&self) -> String {
        self.inner.out_val.clone()
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "CompuTab", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyCompuTabView {
    inner: RustCompuTab,
}

impl PyCompuTabView {
    fn new(inner: RustCompuTab) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCompuTabView {
    #[getter]
    fn name(&self) -> String {
        self.inner.get_name().to_string()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner.long_identifier.clone()
    }

    #[gen_stub(override_return_type(type_repr = "ConversionType"))]
    #[getter]
    fn conversion_type(&self) -> PyConversionType {
        PyConversionType::from(&self.inner.conversion_type)
    }

    #[getter]
    fn number_value_pairs(&self) -> u16 {
        self.inner.number_value_pairs
    }

    #[gen_stub(override_return_type(type_repr = "list[TabEntry]"))]
    #[getter]
    fn entries(&self) -> Vec<PyTabEntryView> {
        self.inner
            .tab_entry
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn default_value(&self) -> Option<String> {
        self.inner
            .default_value
            .as_ref()
            .map(|value| value.display_string.clone())
    }

    #[getter]
    fn default_value_numeric(&self) -> Option<f64> {
        self.inner
            .default_value_numeric
            .as_ref()
            .map(|value| value.display_value)
    }

    #[gen_stub(skip)]
    fn __repr__(&self) -> String {
        format!(
            "CompuTab(name={:?}, entries={})",
            self.inner.get_name(),
            self.inner.tab_entry.len()
        )
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "CompuVtab", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyCompuVtabView {
    inner: RustCompuVtab,
}

impl PyCompuVtabView {
    fn new(inner: RustCompuVtab) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCompuVtabView {
    #[getter]
    fn name(&self) -> String {
        self.inner.get_name().to_string()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner.long_identifier.clone()
    }

    #[gen_stub(override_return_type(type_repr = "ConversionType"))]
    #[getter]
    fn conversion_type(&self) -> PyConversionType {
        PyConversionType::from(&self.inner.conversion_type)
    }

    #[getter]
    fn number_value_pairs(&self) -> u16 {
        self.inner.number_value_pairs
    }

    #[gen_stub(override_return_type(type_repr = "list[ValuePair]"))]
    #[getter]
    fn entries(&self) -> Vec<PyValuePairView> {
        self.inner
            .value_pairs
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn default_value(&self) -> Option<String> {
        self.inner
            .default_value
            .as_ref()
            .map(|value| value.display_string.clone())
    }
}

#[gen_stub_pyclass]
#[pyclass(
    name = "CompuVtabRange",
    module = "a2lfile._a2lfile",
    skip_from_py_object
)]
#[derive(Clone)]
struct PyCompuVtabRangeView {
    inner: RustCompuVtabRange,
}

impl PyCompuVtabRangeView {
    fn new(inner: RustCompuVtabRange) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCompuVtabRangeView {
    #[getter]
    fn name(&self) -> String {
        self.inner.get_name().to_string()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner.long_identifier.clone()
    }

    #[getter]
    fn number_value_triples(&self) -> u16 {
        self.inner.number_value_triples
    }

    #[gen_stub(override_return_type(type_repr = "list[ValueTriple]"))]
    #[getter]
    fn entries(&self) -> Vec<PyValueTripleView> {
        self.inner
            .value_triples
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn default_value(&self) -> Option<String> {
        self.inner
            .default_value
            .as_ref()
            .map(|value| value.display_string.clone())
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "IfData", module = "a2lfile._a2lfile", skip_from_py_object)]
#[derive(Clone)]
struct PyIfDataView {
    inner: RustIfData,
}

impl From<RustIfData> for PyIfDataView {
    fn from(inner: RustIfData) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyIfDataView {
    #[getter]
    fn valid(&self) -> bool {
        self.inner.ifdata_valid
    }

    #[gen_stub(override_return_type(type_repr = "GenericIfData | None"))]
    #[getter]
    fn items(&self) -> Option<PyGenericIfDataView> {
        self.inner.ifdata_items.clone().map(Into::into)
    }

    #[gen_stub(skip)]
    fn __repr__(&self) -> String {
        format!(
            "IfData(valid={}, has_items={})",
            self.inner.ifdata_valid,
            self.inner.ifdata_items.is_some()
        )
    }
}

#[gen_stub_pyclass]
#[pyclass(
    name = "GenericIfData",
    module = "a2lfile._a2lfile",
    skip_from_py_object
)]
#[derive(Clone)]
struct PyGenericIfDataView {
    inner: GenericIfData,
}

impl From<GenericIfData> for PyGenericIfDataView {
    fn from(inner: GenericIfData) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGenericIfDataView {
    #[getter]
    fn kind(&self) -> &'static str {
        match &self.inner {
            GenericIfData::None => "none",
            GenericIfData::Char(..) => "char",
            GenericIfData::Int(..) => "int",
            GenericIfData::Long(..) => "long",
            GenericIfData::Int64(..) => "int64",
            GenericIfData::UChar(..) => "uchar",
            GenericIfData::UInt(..) => "uint",
            GenericIfData::ULong(..) => "ulong",
            GenericIfData::UInt64(..) => "uint64",
            GenericIfData::Float(..) => "float",
            GenericIfData::Double(..) => "double",
            GenericIfData::String(..) => "string",
            GenericIfData::Array(..) => "array",
            GenericIfData::EnumItem(..) => "enum_item",
            GenericIfData::Sequence(..) => "sequence",
            GenericIfData::TaggedStruct(..) => "tagged_struct",
            GenericIfData::TaggedUnion(..) => "tagged_union",
            GenericIfData::Struct(..) => "struct",
            GenericIfData::Block { .. } => "block",
        }
    }

    #[getter]
    fn line(&self) -> Option<u32> {
        match &self.inner {
            GenericIfData::Char(line, _)
            | GenericIfData::Int(line, _)
            | GenericIfData::Long(line, _)
            | GenericIfData::Int64(line, _)
            | GenericIfData::UChar(line, _)
            | GenericIfData::UInt(line, _)
            | GenericIfData::ULong(line, _)
            | GenericIfData::UInt64(line, _)
            | GenericIfData::Float(line, _)
            | GenericIfData::Double(line, _)
            | GenericIfData::String(line, _)
            | GenericIfData::EnumItem(line, _)
            | GenericIfData::Struct(_, line, _)
            | GenericIfData::Block { line, .. } => Some(*line),
            GenericIfData::None
            | GenericIfData::Array(_)
            | GenericIfData::Sequence(_)
            | GenericIfData::TaggedStruct(_)
            | GenericIfData::TaggedUnion(_) => None,
        }
    }

    #[getter]
    fn incfile(&self) -> Option<String> {
        match &self.inner {
            GenericIfData::Struct(incfile, _, _) | GenericIfData::Block { incfile, .. } => {
                incfile.clone()
            }
            _ => None,
        }
    }

    #[getter]
    fn is_hex(&self) -> Option<bool> {
        match &self.inner {
            GenericIfData::Char(_, (_, is_hex))
            | GenericIfData::Int(_, (_, is_hex))
            | GenericIfData::Long(_, (_, is_hex))
            | GenericIfData::Int64(_, (_, is_hex))
            | GenericIfData::UChar(_, (_, is_hex))
            | GenericIfData::UInt(_, (_, is_hex))
            | GenericIfData::ULong(_, (_, is_hex))
            | GenericIfData::UInt64(_, (_, is_hex)) => Some(*is_hex),
            _ => None,
        }
    }

    #[gen_stub(override_return_type(type_repr = "typing.Any | None", imports = ("typing",)))]
    #[getter]
    fn value(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match &self.inner {
            GenericIfData::None => Ok(py.None()),
            GenericIfData::Char(_, (value, _)) => (*value).into_py_any(py),
            GenericIfData::Int(_, (value, _)) => (*value).into_py_any(py),
            GenericIfData::Long(_, (value, _)) => (*value).into_py_any(py),
            GenericIfData::Int64(_, (value, _)) => (*value).into_py_any(py),
            GenericIfData::UChar(_, (value, _)) => (*value).into_py_any(py),
            GenericIfData::UInt(_, (value, _)) => (*value).into_py_any(py),
            GenericIfData::ULong(_, (value, _)) => (*value).into_py_any(py),
            GenericIfData::UInt64(_, (value, _)) => (*value).into_py_any(py),
            GenericIfData::Float(_, value) => (*value).into_py_any(py),
            GenericIfData::Double(_, value) => (*value).into_py_any(py),
            GenericIfData::String(_, value) => value.clone().into_py_any(py),
            GenericIfData::EnumItem(_, value) => value.clone().into_py_any(py),
            GenericIfData::Array(_)
            | GenericIfData::Sequence(_)
            | GenericIfData::TaggedStruct(_)
            | GenericIfData::TaggedUnion(_)
            | GenericIfData::Struct(_, _, _)
            | GenericIfData::Block { .. } => Ok(py.None()),
        }
    }

    #[gen_stub(override_return_type(type_repr = "list[GenericIfData] | None"))]
    #[getter]
    fn items(&self) -> Option<Vec<PyGenericIfDataView>> {
        match &self.inner {
            GenericIfData::Array(items)
            | GenericIfData::Sequence(items)
            | GenericIfData::Struct(_, _, items)
            | GenericIfData::Block { items, .. } => {
                Some(items.iter().cloned().map(Into::into).collect())
            }
            _ => None,
        }
    }

    #[gen_stub(override_return_type(
        type_repr = "dict[str, list[GenericIfDataTaggedItem]] | None"
    ))]
    #[getter]
    fn tagged_items(&self) -> Option<HashMap<String, Vec<PyGenericIfDataTaggedItemView>>> {
        match &self.inner {
            GenericIfData::TaggedStruct(items) | GenericIfData::TaggedUnion(items) => Some(
                items
                    .iter()
                    .map(|(tag, values)| {
                        (
                            tag.clone(),
                            values.iter().cloned().map(Into::into).collect(),
                        )
                    })
                    .collect(),
            ),
            _ => None,
        }
    }

    #[gen_stub(skip)]
    fn __repr__(&self) -> String {
        format!("GenericIfData(kind={:?})", self.kind())
    }
}

#[gen_stub_pyclass]
#[pyclass(
    name = "GenericIfDataTaggedItem",
    module = "a2lfile._a2lfile",
    skip_from_py_object
)]
#[derive(Clone)]
struct PyGenericIfDataTaggedItemView {
    inner: GenericIfDataTaggedItem,
}

impl From<GenericIfDataTaggedItem> for PyGenericIfDataTaggedItemView {
    fn from(inner: GenericIfDataTaggedItem) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGenericIfDataTaggedItemView {
    #[getter]
    fn tag(&self) -> String {
        self.inner.tag.clone()
    }

    #[gen_stub(override_return_type(type_repr = "GenericIfData"))]
    #[getter]
    fn data(&self) -> PyGenericIfDataView {
        self.inner.data.clone().into()
    }

    #[getter]
    fn is_block(&self) -> bool {
        self.inner.is_block
    }

    #[getter]
    fn line(&self) -> u32 {
        self.inner.line
    }

    #[getter]
    fn incfile(&self) -> Option<String> {
        self.inner.incfile.clone()
    }

    #[gen_stub(skip)]
    fn __repr__(&self) -> String {
        format!(
            "GenericIfDataTaggedItem(tag={:?}, is_block={})",
            self.inner.tag, self.inner.is_block
        )
    }
}

#[gen_stub_pyfunction]
#[pyfunction(signature = (path, a2ml_spec=None))]
fn load(path: String, a2ml_spec: Option<String>) -> PyResult<PyA2lFile> {
    let (a2l_file, _) = a2lfile_core::load(path, a2ml_spec, false).map_err(map_a2l_error)?;
    Ok(PyA2lFile {
        // Keep the parsed file shared across Python views instead of duplicating modules.
        inner: Arc::new(a2l_file),
    })
}

#[gen_stub_pyfunction]
#[pyfunction(signature = (text, a2ml_spec=None))]
fn load_from_string(text: &str, a2ml_spec: Option<String>) -> PyResult<PyA2lFile> {
    let (a2l_file, _) =
        a2lfile_core::load_from_string(text, a2ml_spec, false).map_err(map_a2l_error)?;
    Ok(PyA2lFile {
        // Keep the parsed file shared across Python views instead of duplicating modules.
        inner: Arc::new(a2l_file),
    })
}

#[pymodule]
fn _a2lfile(m: &Bound<'_, PyModuleType>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(load_from_string, m)?)?;
    // Register the typed A2L value domains before views return their members.
    m.add_class::<PyAddrType>()?;
    m.add_class::<PyA2lFile>()?;
    m.add_class::<PyAnnotationView>()?;
    m.add_class::<PyBitOperationView>()?;
    m.add_class::<PyCoeffsView>()?;
    m.add_class::<PyCoeffsLinearView>()?;
    m.add_class::<PyByteOrderEnum>()?;
    m.add_class::<PyCompuMethodView>()?;
    m.add_class::<PyCompuTabView>()?;
    m.add_class::<PyCompuVtabView>()?;
    m.add_class::<PyCompuVtabRangeView>()?;
    m.add_class::<PyConversionType>()?;
    m.add_class::<PyDataType>()?;
    m.add_class::<PyFormulaView>()?;
    m.add_class::<PyGenericIfDataView>()?;
    m.add_class::<PyGenericIfDataTaggedItemView>()?;
    m.add_class::<PyIfDataView>()?;
    m.add_class::<PyIndexMode>()?;
    m.add_class::<PyMaxRefreshView>()?;
    m.add_class::<PyMeasurementView>()?;
    m.add_class::<PyModuleView>()?;
    m.add_class::<PySiExponentsView>()?;
    m.add_class::<PySymbolLinkView>()?;
    m.add_class::<PyTabEntryView>()?;
    m.add_class::<PyUnitConversionView>()?;
    m.add_class::<PyUnitType>()?;
    m.add_class::<PyUnitView>()?;
    m.add_class::<PyValuePairView>()?;
    m.add_class::<PyValueTripleView>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
