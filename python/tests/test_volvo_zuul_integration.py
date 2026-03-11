from __future__ import annotations

import json
from dataclasses import asdict, dataclass
from pathlib import Path

import pytest

import a2lfile


VOLVO_ZUUL_A2L = Path(
    "/home/niradynamics.local/maxide/Repos/NdrConfig/Volvo/a2l/"
    "zcla_platform_zuul_b9a8004_FINAL.a2l"
)
COMPOSITE_KINDS = {"struct", "block", "tagged_struct", "tagged_union", "array", "sequence"}
XCP_ROOT_TAGS = {"XCP", "XCPplus"}
TRANSPORT_TAG_PREFIX = "XCP_ON_"


@dataclass(frozen=True)
class ProtocolLayerSummary:
    version: int | None
    max_cto: int | None
    max_dto: int | None
    byte_order: str | None
    address_granularity: str | None
    optional_commands: tuple[str, ...]


@dataclass(frozen=True)
class DaqSummary:
    config_type: str | None
    max_daq: int | None
    max_event_channel: int | None
    max_odt_entry_size_daq: int | None


@dataclass(frozen=True)
class TransportSummary:
    name: str
    version: int | None
    port: int | None
    address: str | None


@dataclass(frozen=True)
class ModuleSummary:
    name: str
    signal_names: tuple[str, ...]
    protocol_layer: ProtocolLayerSummary | None
    daq: DaqSummary | None
    transports: tuple[TransportSummary, ...]


@dataclass(frozen=True)
class InventorySummary:
    source: str
    module_count: int
    modules: tuple[ModuleSummary, ...]

    def to_dict(self) -> dict[str, object]:
        return asdict(self)


def _iter_tagged_items(node: a2lfile.GenericIfData):
    for tag_name in sorted((node.tagged_items or {}).keys()):
        for tagged_item in node.tagged_items[tag_name]:
            yield tagged_item
            yield from _iter_tagged_items(tagged_item.data)

    for child in node.items or []:
        yield from _iter_tagged_items(child)


def _find_first_tagged_item(node: a2lfile.GenericIfData, tag: str):
    for tagged_item in _iter_tagged_items(node):
        if tagged_item.tag == tag:
            return tagged_item
    return None


def _scalar_values(node: a2lfile.GenericIfData) -> list[object]:
    return [
        child.value
        for child in (node.items or [])
        if child.kind not in COMPOSITE_KINDS and child.value is not None
    ]


def _scalar_value_at(node: a2lfile.GenericIfData, index: int) -> object | None:
    items = node.items or []
    if index >= len(items):
        return None

    item = items[index]
    if item.kind in COMPOSITE_KINDS:
        return None

    return item.value


def _iter_xcp_roots(module: a2lfile.Module):
    for if_data in module.if_data:
        if not if_data.valid or if_data.items is None:
            continue

        for tagged_item in _iter_tagged_items(if_data.items):
            if tagged_item.tag in XCP_ROOT_TAGS:
                yield tagged_item.data


def _extract_protocol_layer(xcp_root: a2lfile.GenericIfData) -> ProtocolLayerSummary | None:
    protocol_layer = _find_first_tagged_item(xcp_root, "PROTOCOL_LAYER")
    if protocol_layer is None:
        return None

    commands = sorted(
        {
            value
            for tagged_item in _iter_tagged_items(protocol_layer.data)
            if tagged_item.tag == "OPTIONAL_CMD"
            for value in _scalar_values(tagged_item.data)
            if isinstance(value, str)
        }
    )

    return ProtocolLayerSummary(
        version=_scalar_value_at(protocol_layer.data, 0),
        max_cto=_scalar_value_at(protocol_layer.data, 1),
        max_dto=_scalar_value_at(protocol_layer.data, 2),
        byte_order=_scalar_value_at(protocol_layer.data, 10),
        address_granularity=_scalar_value_at(protocol_layer.data, 11),
        optional_commands=tuple(commands),
    )


def _extract_daq_summary(xcp_root: a2lfile.GenericIfData) -> DaqSummary | None:
    daq = _find_first_tagged_item(xcp_root, "DAQ")
    if daq is None:
        return None

    return DaqSummary(
        config_type=_scalar_value_at(daq.data, 0),
        max_daq=_scalar_value_at(daq.data, 1),
        max_event_channel=_scalar_value_at(daq.data, 2),
        max_odt_entry_size_daq=_scalar_value_at(daq.data, 8),
    )


def _extract_transports(xcp_root: a2lfile.GenericIfData) -> tuple[TransportSummary, ...]:
    transports = {}

    for tagged_item in _iter_tagged_items(xcp_root):
        if not tagged_item.tag.startswith(TRANSPORT_TAG_PREFIX):
            continue

        transport_header = (tagged_item.data.items or [None])[0]
        version = None
        port = None
        address = None

        if transport_header is not None:
            version = _scalar_value_at(transport_header, 0)
            port = _scalar_value_at(transport_header, 1)

            address_item = _find_first_tagged_item(transport_header, "ADDRESS")
            if address_item is not None:
                address_values = _scalar_values(address_item.data)
                address = next(
                    (value for value in address_values if isinstance(value, str)),
                    None,
                )

        transports[tagged_item.tag] = TransportSummary(
            name=tagged_item.tag,
            version=version,
            port=port,
            address=address,
        )

    return tuple(transports[name] for name in sorted(transports))


def _extract_module_summary(module: a2lfile.Module) -> ModuleSummary:
    protocol_layer = None
    daq = None
    transports = {}

    for xcp_root in _iter_xcp_roots(module):
        protocol_layer = protocol_layer or _extract_protocol_layer(xcp_root)
        daq = daq or _extract_daq_summary(xcp_root)

        for transport in _extract_transports(xcp_root):
            transports[transport.name] = transport

    return ModuleSummary(
        name=module.name,
        signal_names=tuple(sorted(measurement.name for measurement in module.measurements)),
        protocol_layer=protocol_layer,
        daq=daq,
        transports=tuple(transports[name] for name in sorted(transports)),
    )


def build_inventory_summary(a2l_path: Path) -> InventorySummary:
    parsed = a2lfile.load(str(a2l_path))
    modules = tuple(sorted((_extract_module_summary(module) for module in parsed.modules), key=lambda item: item.name))

    return InventorySummary(
        source=str(a2l_path),
        module_count=len(modules),
        modules=modules,
    )


@pytest.mark.skipif(
    not VOLVO_ZUUL_A2L.exists(),
    reason=f"Integration A2L not available: {VOLVO_ZUUL_A2L}",
)
def test_volvo_zuul_summary_can_be_built_and_serialized(tmp_path: Path) -> None:
    summary = build_inventory_summary(VOLVO_ZUUL_A2L)

    summary_path = tmp_path / "volvo_zuul_inventory_summary.json"
    summary_path.write_text(json.dumps(summary.to_dict(), indent=2), encoding="utf-8")

    assert summary_path.is_file()
    assert summary.module_count == 1

    module = summary.modules[0]
    assert module.name == "module__ARPackage_EcucValueCollection"
    assert module.signal_names == tuple(sorted(module.signal_names))
    assert module.signal_names

    assert module.protocol_layer is not None
    assert module.protocol_layer.optional_commands
    assert {"GET_ID", "SET_MTA", "ALLOC_DAQ"}.issubset(module.protocol_layer.optional_commands)

    assert module.daq is not None
    assert module.daq.config_type == "DYNAMIC"
    assert module.daq.max_daq == 2

    transport_names = tuple(transport.name for transport in module.transports)
    assert transport_names == ("XCP_ON_UDP_IP",)


@pytest.mark.skipif(
    not VOLVO_ZUUL_A2L.exists(),
    reason=f"Integration A2L not available: {VOLVO_ZUUL_A2L}",
)
def test_volvo_zuul_measurement_scaling_is_resolved_via_compu_method() -> None:
    parsed = a2lfile.load(str(VOLVO_ZUUL_A2L))
    module = parsed.modules[0]
    measurement = module.get_measurement("diagcmnMgr_OutsideTemperatureGSdiagcmnMgr")

    assert isinstance(measurement, a2lfile.Measurement)
    assert measurement.compu_method is not None
    assert measurement.compu_method.conversion_type == "LINEAR"
    assert measurement.compu_method.coeffs_linear is not None
    assert measurement.compu_method.coeffs_linear.a == 0.1
    assert measurement.compu_method.coeffs_linear.b == 0.0
    assert measurement.compu_method.referenced_unit is not None
    assert measurement.compu_method.referenced_unit.display == "degC"
