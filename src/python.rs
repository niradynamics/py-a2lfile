#![allow(unsafe_op_in_unsafe_fn)]

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyModule as PythonModule;
use pyo3::wrap_pyfunction;
use pyo3_stub_gen::derive::{
    gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pyfunction, gen_stub_pymethods,
};
use pyo3_stub_gen::{StubGenConfig, StubInfo};

use a2lfile_core::{
    A2lFile as RustA2lFile, A2lObjectName, AddrType as RustAddrType, Annotation as RustAnnotation,
    BitOperation as RustBitOperation, ByteOrderEnum as RustByteOrderEnum, Coeffs as RustCoeffs,
    CoeffsLinear as RustCoeffsLinear, CompuMethod as RustCompuMethod, CompuTab as RustCompuTab,
    CompuVtab as RustCompuVtab, CompuVtabRange as RustCompuVtabRange,
    ConversionType as RustConversionType, DataType as RustDataType, Formula as RustFormula,
    GenericIfData as RustGenericIfData, GenericIfDataTaggedItem as RustGenericIfDataTaggedItem,
    IfData as RustIfData, IndexMode as RustIndexMode, MaxRefresh as RustMaxRefresh,
    Measurement as RustMeasurement, Module as RustModule, SiExponents as RustSiExponents,
    SymbolLink as RustSymbolLink, TabEntryStruct as RustTabEntry, Unit as RustUnit,
    UnitConversion as RustUnitConversion, UnitType as RustUnitType,
    ValuePairsStruct as RustValuePair, ValueTriplesStruct as RustValueTriple,
};

pyo3_stub_gen::module_variable!("_a2lfile", "__version__", String);

fn map_a2l_error(err: a2lfile_core::A2lError) -> PyErr {
    PyException::new_err(err.to_string())
}

// Mirror the upstream unit enums so Python callers can use typed A2L values.
#[gen_stub_pyclass_enum]
#[pyclass(eq, frozen, hash, skip_from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AddrType {
    Pbyte,
    Pword,
    Plong,
    Plonglong,
    Direct,
}

impl From<&RustAddrType> for AddrType {
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
#[pyclass(eq, frozen, hash, skip_from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ByteOrderEnum {
    LittleEndian,
    BigEndian,
    MsbLast,
    MsbFirst,
    MsbFirstMswLast,
    MsbLastMswFirst,
}

impl From<&RustByteOrderEnum> for ByteOrderEnum {
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
#[pyclass(eq, frozen, hash, skip_from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ConversionType {
    Identical,
    Form,
    Linear,
    RatFunc,
    TabIntp,
    TabNointp,
    TabVerb,
}

impl From<&RustConversionType> for ConversionType {
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
#[pyclass(eq, frozen, hash, skip_from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum DataType {
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

impl From<&RustDataType> for DataType {
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
#[pyclass(eq, frozen, hash, skip_from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum IndexMode {
    AlternateCurves,
    AlternateWithX,
    AlternateWithY,
    ColumnDir,
    RowDir,
}

impl From<&RustIndexMode> for IndexMode {
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
#[pyclass(eq, frozen, hash, skip_from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum UnitType {
    Derived,
    ExtendedSi,
}

impl From<&RustUnitType> for UnitType {
    fn from(value: &RustUnitType) -> Self {
        match value {
            RustUnitType::Derived => Self::Derived,
            RustUnitType::ExtendedSi => Self::ExtendedSi,
        }
    }
}

#[derive(Clone)]
struct ModuleContext {
    a2l: Arc<RustA2lFile>,
    module_index: usize,
}

impl ModuleContext {
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

    fn get_measurement(&self, name: &str) -> Option<&RustMeasurement> {
        self.module().measurement.get(name)
    }

    fn get_compu_method(&self, name: &str) -> Option<RustCompuMethod> {
        if name == "NO_COMPU_METHOD" {
            return None;
        }
        self.module().compu_method.get(name).cloned()
    }

    fn get_unit(&self, name: &str) -> Option<RustUnit> {
        self.module().unit.get(name).cloned()
    }

    fn get_compu_table(&self, name: &str) -> Option<CompuTable> {
        // Look up conversion tables lazily from the shared module data.
        if let Some(compu_tab) = self.module().compu_tab.get(name) {
            return Some(CompuTable::CompuTab(compu_tab.clone()));
        }
        if let Some(compu_vtab) = self.module().compu_vtab.get(name) {
            return Some(CompuTable::CompuVtab(compu_vtab.clone()));
        }
        if let Some(compu_vtab_range) = self.module().compu_vtab_range.get(name) {
            return Some(CompuTable::CompuVtabRange(compu_vtab_range.clone()));
        }
        None
    }
}

enum CompuTable {
    CompuTab(RustCompuTab),
    CompuVtab(RustCompuVtab),
    CompuVtabRange(RustCompuVtabRange),
}

fn wrap_compu_table(py: Python<'_>, table: CompuTable) -> PyResult<Py<PyAny>> {
    match table {
        CompuTable::CompuTab(inner) => Ok(Py::new(py, CompuTab::new(inner))?.into_any()),
        CompuTable::CompuVtab(inner) => Ok(Py::new(py, CompuVtab::new(inner))?.into_any()),
        CompuTable::CompuVtabRange(inner) => {
            Ok(Py::new(py, CompuVtabRange::new(inner))?.into_any())
        }
    }
}

fn wrap_compu_table_by_name(
    py: Python<'_>,
    context: &ModuleContext,
    name: &str,
) -> PyResult<Option<Py<PyAny>>> {
    context
        .get_compu_table(name)
        .map(|table| wrap_compu_table(py, table))
        .transpose()
}

#[gen_stub_pyclass]
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct A2lFile {
    inner: Arc<RustA2lFile>,
}

#[gen_stub_pymethods]
#[pymethods]
impl A2lFile {
    #[getter]
    fn modules(&self) -> Vec<Module> {
        // Expose lightweight module views so Python does not duplicate the full parse tree.
        self.inner
            .project
            .module
            .iter()
            .enumerate()
            .map(|(module_index, _)| Module::new(Arc::clone(&self.inner), module_index))
            .collect()
    }

    #[gen_stub(skip)]
    fn __repr__(&self) -> String {
        format!("A2lFile(modules={})", self.inner.project.module.len())
    }
}

#[gen_stub_pyclass]
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct Module {
    a2l: Arc<RustA2lFile>,
    module_index: usize,
}

impl Module {
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

    fn context(&self) -> Arc<ModuleContext> {
        // Nested views share a lightweight module context instead of cloned lookup tables.
        Arc::new(ModuleContext::new(Arc::clone(&self.a2l), self.module_index))
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Module {
    #[getter]
    fn name(&self) -> String {
        self.inner().get_name().to_string()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner().long_identifier.clone()
    }

    #[getter]
    fn measurements(&self) -> Vec<Measurement> {
        // Reuse one module context for the full returned measurement list.
        let context = self.context();
        self.inner()
            .measurement
            .iter()
            .map(|inner| Measurement::new(inner.get_name().to_string(), Arc::clone(&context)))
            .collect()
    }

    #[getter]
    fn compu_methods(&self) -> Vec<CompuMethod> {
        // Reuse one module context for the full returned conversion list.
        let context = self.context();
        self.inner()
            .compu_method
            .iter()
            .cloned()
            .map(|inner| CompuMethod::new(inner, Arc::clone(&context)))
            .collect()
    }

    #[getter]
    fn compu_tabs(&self) -> Vec<CompuTab> {
        self.inner()
            .compu_tab
            .iter()
            .cloned()
            .map(CompuTab::new)
            .collect()
    }

    #[getter]
    fn compu_vtabs(&self) -> Vec<CompuVtab> {
        self.inner()
            .compu_vtab
            .iter()
            .cloned()
            .map(CompuVtab::new)
            .collect()
    }

    #[getter]
    fn compu_vtab_ranges(&self) -> Vec<CompuVtabRange> {
        self.inner()
            .compu_vtab_range
            .iter()
            .cloned()
            .map(CompuVtabRange::new)
            .collect()
    }

    #[getter]
    fn units(&self) -> Vec<Unit> {
        // Reuse one module context for the full returned unit list.
        let context = self.context();
        self.inner()
            .unit
            .iter()
            .cloned()
            .map(|inner| Unit::new(inner, Arc::clone(&context)))
            .collect()
    }

    #[getter]
    fn if_data(&self) -> Vec<IfData> {
        self.inner()
            .if_data
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn mod_common_byte_order(&self) -> Option<ByteOrderEnum> {
        self.inner()
            .mod_common
            .as_ref()
            .and_then(|value| value.byte_order.as_ref())
            .map(|value| ByteOrderEnum::from(&value.byte_order))
    }

    #[getter]
    fn mod_par_epk(&self) -> Option<String> {
        self.inner()
            .mod_par
            .as_ref()
            .and_then(|value| value.epk.as_ref())
            .map(|value| value.identifier.clone())
    }

    #[getter]
    fn mod_par_addr_epk(&self) -> Vec<u32> {
        self.inner()
            .mod_par
            .as_ref()
            .map(|value| value.addr_epk.iter().map(|addr| addr.address).collect())
            .unwrap_or_default()
    }

    fn get_measurement(&self, name: &str) -> Option<Measurement> {
        // Create context only when returning a nested measurement view.
        let context = self.context();
        self.inner()
            .measurement
            .get(name)
            .map(|inner| Measurement::new(inner.get_name().to_string(), context))
    }

    fn get_compu_method(&self, name: &str) -> Option<CompuMethod> {
        // Look up the conversion from shared module data, then attach the same context.
        let context = self.context();
        context
            .get_compu_method(name)
            .map(|inner| CompuMethod::new(inner, context))
    }

    fn get_unit(&self, name: &str) -> Option<Unit> {
        // Look up the unit from shared module data, then attach the same context.
        let context = self.context();
        context
            .get_unit(name)
            .map(|inner| Unit::new(inner, context))
    }

    #[gen_stub(override_return_type(type_repr = "CompuTab | CompuVtab | CompuVtabRange | None"))]
    fn get_compu_tab(&self, py: Python<'_>, name: &str) -> PyResult<Option<Py<PyAny>>> {
        let context = self.context();
        wrap_compu_table_by_name(py, &context, name)
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct Measurement {
    name: String,
    context: Arc<ModuleContext>,
}

impl Measurement {
    fn new(name: String, context: Arc<ModuleContext>) -> Self {
        Self { name, context }
    }

    fn inner(&self) -> &RustMeasurement {
        // Look up measurement data by name so storing the Python view stays lightweight.
        self.context
            .get_measurement(&self.name)
            .expect("measurement name should stay valid for the lifetime of the parsed file")
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Measurement {
    #[getter]
    fn name(&self) -> String {
        self.name.clone()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner().long_identifier.clone()
    }

    #[getter]
    fn datatype(&self) -> DataType {
        DataType::from(&self.inner().datatype)
    }

    #[getter]
    fn conversion(&self) -> String {
        self.inner().conversion.clone()
    }

    #[getter]
    fn compu_method(&self) -> Option<CompuMethod> {
        self.context
            .get_compu_method(&self.inner().conversion)
            .map(|inner| CompuMethod::new(inner, Arc::clone(&self.context)))
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

    #[getter]
    fn address_type(&self) -> Option<AddrType> {
        self.inner()
            .address_type
            .as_ref()
            .map(|value| AddrType::from(&value.address_type))
    }

    #[getter]
    fn annotation(&self) -> Vec<Annotation> {
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

    #[getter]
    fn bit_operation(&self) -> Option<BitOperation> {
        self.inner().bit_operation.clone().map(Into::into)
    }

    #[getter]
    fn byte_order(&self) -> Option<ByteOrderEnum> {
        self.inner()
            .byte_order
            .as_ref()
            .map(|value| ByteOrderEnum::from(&value.byte_order))
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
    fn function_list(&self) -> Vec<String> {
        self.inner()
            .function_list
            .as_ref()
            .map(|value| value.name_list.clone())
            .unwrap_or_default()
    }

    #[getter]
    fn if_data(&self) -> Vec<IfData> {
        self.inner()
            .if_data
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn layout(&self) -> Option<IndexMode> {
        self.inner()
            .layout
            .as_ref()
            .map(|value| IndexMode::from(&value.index_mode))
    }

    #[getter]
    fn matrix_dim(&self) -> Vec<u16> {
        self.inner()
            .matrix_dim
            .as_ref()
            .map(|value| value.dim_list.clone())
            .unwrap_or_default()
    }

    #[getter]
    fn max_refresh(&self) -> Option<MaxRefresh> {
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

    #[getter]
    fn symbol_link(&self) -> Option<SymbolLink> {
        self.inner().symbol_link.clone().map(Into::into)
    }

    #[getter]
    fn r#virtual(&self) -> Vec<String> {
        self.inner()
            .var_virtual
            .as_ref()
            .map(|value| value.measuring_channel_list.clone())
            .unwrap_or_default()
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct Annotation {
    inner: RustAnnotation,
}

impl From<RustAnnotation> for Annotation {
    fn from(inner: RustAnnotation) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Annotation {
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
    fn text_lines(&self) -> Vec<String> {
        self.inner
            .annotation_text
            .as_ref()
            .map(|value| value.annotation_text_list.clone())
            .unwrap_or_default()
    }
}

#[gen_stub_pyclass]
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct BitOperation {
    inner: RustBitOperation,
}

impl From<RustBitOperation> for BitOperation {
    fn from(inner: RustBitOperation) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl BitOperation {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct MaxRefresh {
    inner: RustMaxRefresh,
}

impl From<RustMaxRefresh> for MaxRefresh {
    fn from(inner: RustMaxRefresh) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl MaxRefresh {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct SymbolLink {
    inner: RustSymbolLink,
}

impl From<RustSymbolLink> for SymbolLink {
    fn from(inner: RustSymbolLink) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl SymbolLink {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct Coeffs {
    inner: RustCoeffs,
}

impl From<RustCoeffs> for Coeffs {
    fn from(inner: RustCoeffs) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Coeffs {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct CoeffsLinear {
    inner: RustCoeffsLinear,
}

impl From<RustCoeffsLinear> for CoeffsLinear {
    fn from(inner: RustCoeffsLinear) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl CoeffsLinear {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct Formula {
    inner: RustFormula,
}

impl From<RustFormula> for Formula {
    fn from(inner: RustFormula) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Formula {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct UnitConversion {
    inner: RustUnitConversion,
}

impl From<RustUnitConversion> for UnitConversion {
    fn from(inner: RustUnitConversion) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl UnitConversion {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct SiExponents {
    inner: RustSiExponents,
}

impl From<RustSiExponents> for SiExponents {
    fn from(inner: RustSiExponents) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl SiExponents {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct Unit {
    inner: RustUnit,
    context: Arc<ModuleContext>,
}

impl Unit {
    fn new(inner: RustUnit, context: Arc<ModuleContext>) -> Self {
        Self { inner, context }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Unit {
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

    #[getter]
    fn unit_type(&self) -> UnitType {
        UnitType::from(&self.inner.unit_type)
    }

    #[getter]
    fn ref_unit(&self) -> Option<String> {
        self.inner.ref_unit.as_ref().map(|value| value.unit.clone())
    }

    #[getter]
    fn referenced_unit(&self) -> Option<Unit> {
        self.inner
            .ref_unit
            .as_ref()
            .and_then(|value| self.context.get_unit(&value.unit))
            .map(|inner| Unit::new(inner, Arc::clone(&self.context)))
    }

    #[getter]
    fn si_exponents(&self) -> Option<SiExponents> {
        self.inner.si_exponents.clone().map(Into::into)
    }

    #[getter]
    fn unit_conversion(&self) -> Option<UnitConversion> {
        self.inner.unit_conversion.clone().map(Into::into)
    }
}

#[gen_stub_pyclass]
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct CompuMethod {
    inner: RustCompuMethod,
    context: Arc<ModuleContext>,
}

impl CompuMethod {
    fn new(inner: RustCompuMethod, context: Arc<ModuleContext>) -> Self {
        Self { inner, context }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl CompuMethod {
    #[getter]
    fn name(&self) -> String {
        self.inner.get_name().to_string()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner.long_identifier.clone()
    }

    #[getter]
    fn conversion_type(&self) -> ConversionType {
        ConversionType::from(&self.inner.conversion_type)
    }

    #[getter]
    fn format(&self) -> String {
        self.inner.format.clone()
    }

    #[getter]
    fn unit(&self) -> String {
        self.inner.unit.clone()
    }

    #[getter]
    fn coeffs(&self) -> Option<Coeffs> {
        self.inner.coeffs.clone().map(Into::into)
    }

    #[getter]
    fn coeffs_linear(&self) -> Option<CoeffsLinear> {
        self.inner.coeffs_linear.clone().map(Into::into)
    }

    #[getter]
    fn compu_tab_ref(&self) -> Option<String> {
        self.inner
            .compu_tab_ref
            .as_ref()
            .map(|value| value.conversion_table.clone())
    }

    #[getter]
    fn formula(&self) -> Option<Formula> {
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

    #[getter]
    fn referenced_unit(&self) -> Option<Unit> {
        self.inner
            .ref_unit
            .as_ref()
            .and_then(|value| self.context.get_unit(&value.unit))
            .map(|inner| Unit::new(inner, Arc::clone(&self.context)))
    }

    #[gen_stub(override_return_type(type_repr = "CompuTab | CompuVtab | CompuVtabRange | None"))]
    #[getter]
    fn referenced_table(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        if let Some(compu_tab_ref) = &self.inner.compu_tab_ref {
            return wrap_compu_table_by_name(py, &self.context, &compu_tab_ref.conversion_table);
        }
        Ok(None)
    }
}

#[gen_stub_pyclass]
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct TabEntry {
    inner: RustTabEntry,
}

impl From<RustTabEntry> for TabEntry {
    fn from(inner: RustTabEntry) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl TabEntry {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct ValuePair {
    inner: RustValuePair,
}

impl From<RustValuePair> for ValuePair {
    fn from(inner: RustValuePair) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl ValuePair {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct ValueTriple {
    inner: RustValueTriple,
}

impl From<RustValueTriple> for ValueTriple {
    fn from(inner: RustValueTriple) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl ValueTriple {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct CompuTab {
    inner: RustCompuTab,
}

impl CompuTab {
    fn new(inner: RustCompuTab) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl CompuTab {
    #[getter]
    fn name(&self) -> String {
        self.inner.get_name().to_string()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner.long_identifier.clone()
    }

    #[getter]
    fn conversion_type(&self) -> ConversionType {
        ConversionType::from(&self.inner.conversion_type)
    }

    #[getter]
    fn number_value_pairs(&self) -> u16 {
        self.inner.number_value_pairs
    }

    #[getter]
    fn entries(&self) -> Vec<TabEntry> {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct CompuVtab {
    inner: RustCompuVtab,
}

impl CompuVtab {
    fn new(inner: RustCompuVtab) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl CompuVtab {
    #[getter]
    fn name(&self) -> String {
        self.inner.get_name().to_string()
    }

    #[getter]
    fn long_identifier(&self) -> String {
        self.inner.long_identifier.clone()
    }

    #[getter]
    fn conversion_type(&self) -> ConversionType {
        ConversionType::from(&self.inner.conversion_type)
    }

    #[getter]
    fn number_value_pairs(&self) -> u16 {
        self.inner.number_value_pairs
    }

    #[getter]
    fn entries(&self) -> Vec<ValuePair> {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct CompuVtabRange {
    inner: RustCompuVtabRange,
}

impl CompuVtabRange {
    fn new(inner: RustCompuVtabRange) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl CompuVtabRange {
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

    #[getter]
    fn entries(&self) -> Vec<ValueTriple> {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct IfData {
    inner: RustIfData,
}

impl From<RustIfData> for IfData {
    fn from(inner: RustIfData) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl IfData {
    #[getter]
    fn valid(&self) -> bool {
        self.inner.ifdata_valid
    }

    #[getter]
    fn items(&self) -> Option<GenericIfData> {
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
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct GenericIfData {
    inner: RustGenericIfData,
}

impl From<RustGenericIfData> for GenericIfData {
    fn from(inner: RustGenericIfData) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl GenericIfData {
    #[getter]
    fn kind(&self) -> &'static str {
        match &self.inner {
            RustGenericIfData::None => "none",
            RustGenericIfData::Char(..) => "char",
            RustGenericIfData::Int(..) => "int",
            RustGenericIfData::Long(..) => "long",
            RustGenericIfData::Int64(..) => "int64",
            RustGenericIfData::UChar(..) => "uchar",
            RustGenericIfData::UInt(..) => "uint",
            RustGenericIfData::ULong(..) => "ulong",
            RustGenericIfData::UInt64(..) => "uint64",
            RustGenericIfData::Float(..) => "float",
            RustGenericIfData::Double(..) => "double",
            RustGenericIfData::String(..) => "string",
            RustGenericIfData::Array(..) => "array",
            RustGenericIfData::EnumItem(..) => "enum_item",
            RustGenericIfData::Sequence(..) => "sequence",
            RustGenericIfData::TaggedStruct(..) => "tagged_struct",
            RustGenericIfData::TaggedUnion(..) => "tagged_union",
            RustGenericIfData::Struct(..) => "struct",
            RustGenericIfData::Block { .. } => "block",
        }
    }

    #[getter]
    fn line(&self) -> Option<u32> {
        match &self.inner {
            RustGenericIfData::Char(line, _)
            | RustGenericIfData::Int(line, _)
            | RustGenericIfData::Long(line, _)
            | RustGenericIfData::Int64(line, _)
            | RustGenericIfData::UChar(line, _)
            | RustGenericIfData::UInt(line, _)
            | RustGenericIfData::ULong(line, _)
            | RustGenericIfData::UInt64(line, _)
            | RustGenericIfData::Float(line, _)
            | RustGenericIfData::Double(line, _)
            | RustGenericIfData::String(line, _)
            | RustGenericIfData::EnumItem(line, _)
            | RustGenericIfData::Struct(_, line, _)
            | RustGenericIfData::Block { line, .. } => Some(*line),
            RustGenericIfData::None
            | RustGenericIfData::Array(_)
            | RustGenericIfData::Sequence(_)
            | RustGenericIfData::TaggedStruct(_)
            | RustGenericIfData::TaggedUnion(_) => None,
        }
    }

    #[getter]
    fn incfile(&self) -> Option<String> {
        match &self.inner {
            RustGenericIfData::Struct(incfile, _, _) | RustGenericIfData::Block { incfile, .. } => {
                incfile.clone()
            }
            _ => None,
        }
    }

    #[getter]
    fn is_hex(&self) -> Option<bool> {
        match &self.inner {
            RustGenericIfData::Char(_, (_, is_hex))
            | RustGenericIfData::Int(_, (_, is_hex))
            | RustGenericIfData::Long(_, (_, is_hex))
            | RustGenericIfData::Int64(_, (_, is_hex))
            | RustGenericIfData::UChar(_, (_, is_hex))
            | RustGenericIfData::UInt(_, (_, is_hex))
            | RustGenericIfData::ULong(_, (_, is_hex))
            | RustGenericIfData::UInt64(_, (_, is_hex)) => Some(*is_hex),
            _ => None,
        }
    }

    #[getter]
    fn value(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match &self.inner {
            RustGenericIfData::None => Ok(py.None()),
            RustGenericIfData::Char(_, (value, _)) => (*value).into_py_any(py),
            RustGenericIfData::Int(_, (value, _)) => (*value).into_py_any(py),
            RustGenericIfData::Long(_, (value, _)) => (*value).into_py_any(py),
            RustGenericIfData::Int64(_, (value, _)) => (*value).into_py_any(py),
            RustGenericIfData::UChar(_, (value, _)) => (*value).into_py_any(py),
            RustGenericIfData::UInt(_, (value, _)) => (*value).into_py_any(py),
            RustGenericIfData::ULong(_, (value, _)) => (*value).into_py_any(py),
            RustGenericIfData::UInt64(_, (value, _)) => (*value).into_py_any(py),
            RustGenericIfData::Float(_, value) => (*value).into_py_any(py),
            RustGenericIfData::Double(_, value) => (*value).into_py_any(py),
            RustGenericIfData::String(_, value) => value.clone().into_py_any(py),
            RustGenericIfData::EnumItem(_, value) => value.clone().into_py_any(py),
            RustGenericIfData::Array(_)
            | RustGenericIfData::Sequence(_)
            | RustGenericIfData::TaggedStruct(_)
            | RustGenericIfData::TaggedUnion(_)
            | RustGenericIfData::Struct(_, _, _)
            | RustGenericIfData::Block { .. } => Ok(py.None()),
        }
    }

    #[getter]
    fn items(&self) -> Vec<GenericIfData> {
        match &self.inner {
            RustGenericIfData::Array(items)
            | RustGenericIfData::Sequence(items)
            | RustGenericIfData::Struct(_, _, items)
            | RustGenericIfData::Block { items, .. } => {
                items.iter().cloned().map(Into::into).collect()
            }
            _ => Vec::new(),
        }
    }

    #[getter]
    fn tagged_items(&self) -> HashMap<String, Vec<GenericIfDataTaggedItem>> {
        match &self.inner {
            RustGenericIfData::TaggedStruct(items) | RustGenericIfData::TaggedUnion(items) => items
                .iter()
                .map(|(tag, values)| {
                    (
                        tag.clone(),
                        values.iter().cloned().map(Into::into).collect(),
                    )
                })
                .collect(),
            _ => HashMap::new(),
        }
    }

    /// Return direct tagged entries for `tag` without converting the complete tag map.
    fn get_tagged_items_by(&self, tag: &str) -> Vec<GenericIfDataTaggedItem> {
        match &self.inner {
            RustGenericIfData::TaggedStruct(items) | RustGenericIfData::TaggedUnion(items) => {
                // Look up only the requested direct child instead of converting the complete map.
                items
                    .get(tag)
                    .into_iter()
                    .flatten()
                    .cloned()
                    .map(Into::into)
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    #[gen_stub(skip)]
    fn __repr__(&self) -> String {
        format!("GenericIfData(kind={:?})", self.kind())
    }
}

#[gen_stub_pyclass]
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct GenericIfDataTaggedItem {
    inner: RustGenericIfDataTaggedItem,
}

impl From<RustGenericIfDataTaggedItem> for GenericIfDataTaggedItem {
    fn from(inner: RustGenericIfDataTaggedItem) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl GenericIfDataTaggedItem {
    #[getter]
    fn tag(&self) -> String {
        self.inner.tag.clone()
    }

    #[getter]
    fn data(&self) -> GenericIfData {
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
fn load(path: String, a2ml_spec: Option<String>) -> PyResult<A2lFile> {
    let (a2l_file, _) = a2lfile_core::load(path, a2ml_spec, false).map_err(map_a2l_error)?;
    Ok(A2lFile {
        // Keep the parsed file shared across Python views instead of duplicating modules.
        inner: Arc::new(a2l_file),
    })
}

#[gen_stub_pyfunction]
#[pyfunction(signature = (text, a2ml_spec=None))]
fn load_from_string(text: &str, a2ml_spec: Option<String>) -> PyResult<A2lFile> {
    let (a2l_file, _) =
        a2lfile_core::load_from_string(text, a2ml_spec, false).map_err(map_a2l_error)?;
    Ok(A2lFile {
        // Keep the parsed file shared across Python views instead of duplicating modules.
        inner: Arc::new(a2l_file),
    })
}

#[pymodule]
fn _a2lfile(m: &Bound<'_, PythonModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(load_from_string, m)?)?;
    // Register the typed A2L value domains before views return their members.
    m.add_class::<AddrType>()?;
    m.add_class::<A2lFile>()?;
    m.add_class::<Annotation>()?;
    m.add_class::<BitOperation>()?;
    m.add_class::<Coeffs>()?;
    m.add_class::<CoeffsLinear>()?;
    m.add_class::<ByteOrderEnum>()?;
    m.add_class::<CompuMethod>()?;
    m.add_class::<CompuTab>()?;
    m.add_class::<CompuVtab>()?;
    m.add_class::<CompuVtabRange>()?;
    m.add_class::<ConversionType>()?;
    m.add_class::<DataType>()?;
    m.add_class::<Formula>()?;
    m.add_class::<GenericIfData>()?;
    m.add_class::<GenericIfDataTaggedItem>()?;
    m.add_class::<IfData>()?;
    m.add_class::<IndexMode>()?;
    m.add_class::<MaxRefresh>()?;
    m.add_class::<Measurement>()?;
    m.add_class::<Module>()?;
    m.add_class::<SiExponents>()?;
    m.add_class::<SymbolLink>()?;
    m.add_class::<TabEntry>()?;
    m.add_class::<UnitConversion>()?;
    m.add_class::<UnitType>()?;
    m.add_class::<Unit>()?;
    m.add_class::<ValuePair>()?;
    m.add_class::<ValueTriple>()?;
    Ok(())
}

/// Build stub metadata for the native module's module-stub layout.
pub fn stub_info() -> pyo3_stub_gen::Result<StubInfo> {
    let python_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("python")
        .join("a2lfile");

    StubInfo::from_project_root(
        "_a2lfile".to_string(),
        python_root,
        false,
        StubGenConfig::default(),
    )
}
