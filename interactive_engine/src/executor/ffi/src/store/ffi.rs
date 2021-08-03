//! Copyright 2020 Alibaba Group Holding Limited.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use std::os::raw::c_void;

use maxgraph_store::v2::api::*;
use maxgraph_store::v2::errors::Error;
use crate::store::graph::{PartitionGraphHandle, FfiPartitionGraph};
use maxgraph_store::v2::wrapper::wrapper_partition_graph::WrapperPartitionSnapshot;
use maxgraph_store::v2::wrapper::graph_storage::{GraphStorageWrapper, WrapperVertex, WrapperEdge, WrapperProperty};
use maxgraph_store::db::graph::store::GraphStore;
use maxgraph_store::v2::api::partition_graph::PartitionGraph;
use maxgraph_store::v2::Result;
use std::vec::IntoIter;

pub type PartitionSnapshotHandle = *const c_void;
pub type ErrorHandle = *const c_void;
pub type VertexHandle = *const c_void;
pub type VertexIteratorHandle = *const c_void;
pub type EdgeHandle = *const c_void;
pub type EdgeIteratorHandle = *const c_void;
pub type PropertyHandle = *const c_void;
pub type PropertyIteratorHandle = *const c_void;

// pub type FfiMultiVersionGraph = WrapperPartitionGraph<GraphStorageWrapper<GraphStore>>;
pub type FfiSnapshot = WrapperPartitionSnapshot<GraphStorageWrapper<GraphStore>>;
pub type FfiVertex = WrapperVertex;
pub type FfiEdge = WrapperEdge;
pub type FfiVertexIterator = IntoIter<Result<FfiVertex>>;
pub type FfiEdgeIterator = IntoIter<Result<FfiEdge>>;
pub type FfiProperty = WrapperProperty;
pub type FfiPropertyIterator = IntoIter<Result<FfiProperty>>;

#[repr(C)]
pub struct StringSlice {
    data: *const u8,
    len: usize,
}

impl StringSlice {
    pub fn new(data : *const u8, len : usize) -> StringSlice {
        StringSlice {
            data,
            len
        }
    }

    pub fn null() -> StringSlice {
        StringSlice {
            data : std::ptr::null(),
            len : 0
        }
    }
}

/// Partition Snapshot FFIs

#[no_mangle]
pub extern fn GetSnapshot(handle: PartitionGraphHandle, snapshot_id: SnapshotId) -> PartitionSnapshotHandle {
    let graph_store_ptr = unsafe { &*(handle as *const FfiPartitionGraph) };
    let snapshot = graph_store_ptr.get_snapshot(snapshot_id);
    Box::into_raw(Box::new(snapshot)) as PartitionSnapshotHandle
}

#[no_mangle]
pub extern fn GetVertex(partition_snapshot: PartitionSnapshotHandle,
                        vertex_id: VertexId, label_id: LabelId,
                        error: *mut ErrorHandle)
                        -> VertexHandle {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        match handler.get_vertex(vertex_id, label_option(label_id), None) {
            Ok(Some(data)) => {
                let data_hdl = Box::new(data);
                return Box::into_raw(data_hdl) as VertexHandle;
            }
            Ok(None) => {}
            Err(e) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
            }
        }
        ::std::ptr::null()
    }
}

#[no_mangle]
pub extern fn GetEdge(partition_snapshot: PartitionSnapshotHandle,
                      edge_id: EdgeId, edge_relation: &EdgeRelation,
                      error: *mut ErrorHandle)
                      -> EdgeHandle {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        match handler.get_edge(edge_id, edge_relation_option(edge_relation), None) {
            Ok(Some(data)) => {
                let data_hdl = Box::new(data);
                return Box::into_raw(data_hdl) as EdgeHandle;
            }
            Ok(None) => {}
            Err(e) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
            }
        }
        ::std::ptr::null()
    }
}

#[no_mangle]
pub extern fn ScanVertex(partition_snapshot: PartitionSnapshotHandle,
                         label_id: LabelId,
                         error: *mut ErrorHandle)
                         -> VertexIteratorHandle {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        match handler.scan_vertex(label_option(label_id), None, None) {
            Ok(data) => {
                Box::into_raw(data) as VertexIteratorHandle
            }
            Err(e) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                ::std::ptr::null()
            }
        }
    }
}

#[no_mangle]
pub extern fn ScanEdge(partition_snapshot: PartitionSnapshotHandle,
                       edge_relation: &EdgeRelation,
                       error: *mut ErrorHandle)
                       -> EdgeIteratorHandle {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        match handler.scan_edge(edge_relation_option(edge_relation), None, None) {
            Ok(data) => {
                Box::into_raw(data) as EdgeIteratorHandle
            }
            Err(e) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                ::std::ptr::null()
            }
        }
    }
}

#[no_mangle]
pub extern fn GetOutEdges(partition_snapshot: PartitionSnapshotHandle,
                          vertex_id: VertexId, edge_label_id: LabelId,
                          error: *mut ErrorHandle)
                          -> EdgeIteratorHandle {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        match handler.get_out_edges(vertex_id, label_option(edge_label_id), None, None) {
            Ok(data) => {
                Box::into_raw(data) as EdgeIteratorHandle
            }
            Err(e) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                ::std::ptr::null()
            }
        }
    }
}

#[no_mangle]
pub extern fn GetInEdges(partition_snapshot: PartitionSnapshotHandle,
                         vertex_id: VertexId, edge_label_id: LabelId,
                         error: *mut ErrorHandle)
                         -> EdgeIteratorHandle {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        match handler.get_in_edges(vertex_id, label_option(edge_label_id), None, None) {
            Ok(data) => {
                Box::into_raw(data) as EdgeIteratorHandle
            }
            Err(e) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                std::ptr::null()
            }
        }
    }
}

#[no_mangle]
pub extern fn GetOutDegree(partition_snapshot: PartitionSnapshotHandle,
                           vertex_id: VertexId, edge_relation: &EdgeRelation,
                           error: *mut ErrorHandle)
                           -> usize {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        match handler.get_out_degree(vertex_id, edge_relation) {
            Ok(degree) => {
                degree
            }
            Err(e) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                usize::MAX
            }
        }
    }
}

#[no_mangle]
pub extern fn GetInDegree(partition_snapshot: PartitionSnapshotHandle,
                          vertex_id: VertexId, edge_relation: &EdgeRelation,
                          error: *mut ErrorHandle)
                          -> usize {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        match handler.get_in_degree(vertex_id, edge_relation) {
            Ok(degree) => {
                degree
            }
            Err(e) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                usize::MAX
            }
        }
    }
}

#[no_mangle]
pub extern fn GetKthOutEdge(partition_snapshot: PartitionSnapshotHandle,
                            vertex_id: VertexId, edge_relation: &EdgeRelation, k: SerialId,
                            error: *mut ErrorHandle)
                            -> EdgeHandle {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        let mut ret : EdgeHandle = ::std::ptr::null();
        match handler.get_kth_out_edge(vertex_id, edge_relation, k, None) {
            Ok(Some(data)) => {
                let data_hdl = Box::new(data);
                ret = Box::into_raw(data_hdl) as EdgeHandle;
            }
            Ok(None) => {}
            Err(e) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
            }
        }
        ret
    }
}

#[no_mangle]
pub extern fn GetKthInEdge(partition_snapshot: PartitionSnapshotHandle,
                           vertex_id: VertexId, edge_relation: &EdgeRelation, k: SerialId,
                           error: *mut ErrorHandle)
                           -> EdgeHandle {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        let mut ret : EdgeHandle = ::std::ptr::null();
        match handler.get_kth_in_edge(vertex_id, edge_relation, k, None) {
            Ok(Some(data)) => {
                let data_hdl = Box::new(data);
                ret = Box::into_raw(data_hdl) as EdgeHandle;
            }
            Ok(None) => {}
            Err(e) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
            }
        }
        ret
    }
}

#[no_mangle]
pub extern fn GetSnapshotId(partition_snapshot: PartitionSnapshotHandle) -> SnapshotId {
    unsafe {
        let handler = &*(partition_snapshot as *const FfiSnapshot);
        handler.get_snapshot_id()
    }
}

/// Vertex FFIs

#[no_mangle]
pub extern fn VertexIteratorNext(vertex_iterator_handle: VertexIteratorHandle,
                                 error: *mut ErrorHandle)
                                 -> VertexHandle {
    unsafe {
        let handler = &mut *(vertex_iterator_handle as *mut FfiVertexIterator);
        match handler.next() {
            Some(Ok(data)) => {
                let data_hdl = Box::new(data);
                return Box::into_raw(data_hdl) as VertexHandle;
            }
            Some(Err(e)) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
            }
            None => {}
        }
        ::std::ptr::null()
    }
}

#[no_mangle]
pub extern fn GetVertexId(vertex_handle: VertexHandle) -> VertexId {
    unsafe {
        let handler = &*(vertex_handle as *const FfiVertex);
        handler.get_vertex_id()
    }
}

#[no_mangle]
pub extern fn GetVertexLabelId(vertex_handle: VertexHandle) -> LabelId {
    unsafe {
        let handler = &*(vertex_handle as *const FfiVertex);
        handler.get_label_id()
    }
}

#[no_mangle]
pub extern fn GetVertexProperty(vertex_handle: VertexHandle, property_id: PropertyId) -> PropertyHandle {
    unsafe {
        let handler = &*(vertex_handle as *const FfiVertex);
        match handler.get_property(property_id) {
            Some(prop) => {
                let prop_hdl = Box::new(prop);
                Box::into_raw(prop_hdl) as PropertyHandle
            }
            None => {
                ::std::ptr::null()
            }
        }
    }
}

#[no_mangle]
pub extern fn GetVertexPropertyIterator(vertex_handle: VertexHandle) -> PropertyIteratorHandle {
    unsafe {
        let handler = &*(vertex_handle as *const FfiVertex);
        let iter_hdl = Box::new(handler.get_property_iterator());
        Box::into_raw(iter_hdl) as PropertyIteratorHandle
    }
}

/// Edge FFIs

#[no_mangle]
pub extern fn EdgeIteratorNext(edge_iterator_handle: EdgeIteratorHandle,
                               error: *mut ErrorHandle)
                               -> EdgeHandle {
    unsafe {
        let handler = &mut *(edge_iterator_handle as *mut FfiEdgeIterator);
        match handler.next() {
            Some(Ok(data)) => {
                let data_hdl = Box::new(data);
                return Box::into_raw(data_hdl) as EdgeHandle;
            }
            Some(Err(e)) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
            }
            None => {}
        }
        ::std::ptr::null()
    }
}

#[no_mangle]
pub extern fn GetEdgeId(edge_handle: EdgeHandle) -> EdgeId {
    unsafe {
        let handler = &*(edge_handle as *const FfiEdge);
        handler.get_edge_id()
    }
}

#[no_mangle]
pub extern fn GetEdgeRelation(edge_handle: EdgeHandle) -> EdgeRelation {
    unsafe {
        let handler = &*(edge_handle as *const FfiEdge);
        handler.get_edge_relation()
    }
}

#[no_mangle]
pub extern fn GetEdgeProperty(edge_handle: EdgeHandle, property_id: PropertyId) -> PropertyHandle {
    unsafe {
        let handler = &*(edge_handle as *const FfiEdge);
        match handler.get_property(property_id) {
            Some(prop) => {
                let prop_hdl = Box::new(prop);
                Box::into_raw(prop_hdl) as PropertyHandle
            }
            None => {
                ::std::ptr::null()
            }
        }
    }
}

#[no_mangle]
pub extern fn GetEdgePropertyIterator(edge_handle: EdgeHandle) -> PropertyIteratorHandle {
    unsafe {
        let handler = &*(edge_handle as *const FfiEdge);
        let iter_hdl = Box::new(handler.get_property_iterator());
        Box::into_raw(iter_hdl) as PropertyIteratorHandle
    }
}

/// Property FFIs

#[no_mangle]
pub extern fn PropertyIteratorNext(property_iterator_handle: PropertyIteratorHandle,
                                   error: *mut ErrorHandle)
                                   -> PropertyHandle {
    unsafe {
        let handler = &mut *(property_iterator_handle as *mut FfiPropertyIterator);
        match handler.next() {
            Some(Ok(data)) => {
                let data_hdl = Box::new(data);
                return Box::into_raw(data_hdl) as PropertyHandle;
            }
            Some(Err(e)) => {
                let error_hdl = Box::new(e);
                *error = Box::into_raw(error_hdl) as ErrorHandle;
            }
            None => {}
        }
        ::std::ptr::null()
    }
}

#[no_mangle]
pub extern fn GetPropertyId(property_handle: PropertyHandle) -> PropertyId {
    unsafe {
        let handler = &*(property_handle as *const FfiProperty);
        handler.get_property_id()
    }
}

#[no_mangle]
pub extern fn GetPropertyAsInt32(property_handle: PropertyHandle, error: *mut ErrorHandle) -> i32 {
    unsafe {
        let handler = &*(property_handle as *const FfiProperty);
        match handler.get_property_value() {
            PropertyValue::Int(data) => {
                *data
            }
            PropertyValue::Null => {
                let error_hdl = Box::new(Error::Internal("None Record!".parse().unwrap()));
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                i32::MAX
            }
            _ => {
                let error_hdl = Box::new(Error::Internal("Wrong Record Type Int32!".parse().unwrap()));
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                i32::MAX
            }
        }
    }
}

#[no_mangle]
pub extern fn GetPropertyAsInt64(property_handle: PropertyHandle, error: *mut ErrorHandle) -> i64 {
    unsafe {
        let handler = &*(property_handle as *const FfiProperty);
        match handler.get_property_value() {
            PropertyValue::Long(data) => {
                *data
            }
            PropertyValue::Null => {
                let error_hdl = Box::new(Error::Internal("None Record!".parse().unwrap()));
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                i64::MAX
            }
            _ => {
                let error_hdl = Box::new(Error::Internal("Wrong Record Type Int64!".parse().unwrap()));
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                i64::MAX
            }
        }
    }
}

#[no_mangle]
pub extern fn GetPropertyAsFloat(property_handle: PropertyHandle, error: *mut ErrorHandle) -> f32 {
    unsafe {
        let handler = &*(property_handle as *const FfiProperty);
        match handler.get_property_value() {
            PropertyValue::Float(data) => {
                *data
            }
            PropertyValue::Null => {
                let error_hdl = Box::new(Error::Internal("None Record!".parse().unwrap()));
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                f32::MAX
            }
            _ => {
                let error_hdl = Box::new(Error::Internal("Wrong Record Type Float!".parse().unwrap()));
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                f32::MAX
            }
        }
    }
}

#[no_mangle]
pub extern fn GetPropertyAsDouble(property_handle: PropertyHandle, error: *mut ErrorHandle) -> f64 {
    unsafe {
        let handler = &*(property_handle as *const FfiProperty);
        match handler.get_property_value() {
            PropertyValue::Double(data) => {
                *data
            }
            PropertyValue::Null => {
                let error_hdl = Box::new(Error::Internal("None Record!".parse().unwrap()));
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                f64::MAX
            }
            _ => {
                let error_hdl = Box::new(Error::Internal("Wrong Record Type Double!".parse().unwrap()));
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                f64::MAX
            }
        }
    }
}

#[no_mangle]
pub extern fn GetPropertyAsString(property_handle: PropertyHandle, error: *mut ErrorHandle) -> StringSlice {
    unsafe {
        let handler = &*(property_handle as *const FfiProperty);
        match handler.get_property_value() {
            PropertyValue::String(data) => {
                StringSlice::new(data.as_ptr(), data.len())
            }
            PropertyValue::Null => {
                let error_hdl = Box::new(Error::Internal("None Record!".parse().unwrap()));
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                StringSlice::null()
            }
            _ => {
                let error_hdl = Box::new(Error::Internal("Wrong Record Type Double!".parse().unwrap()));
                *error = Box::into_raw(error_hdl) as ErrorHandle;
                StringSlice::null()
            }
        }
    }
}

/// Error FFIs

#[no_mangle]
pub extern fn PrintError(error_handle: ErrorHandle) {
    unsafe {
        let handler = &*(error_handle as *const Error);
        handler.what();
    }
}

/// Handle Release FFIs

#[no_mangle]
pub extern fn ReleasePartitionSnapshotHandle(ptr: PartitionSnapshotHandle) {
    println!("[Rust FFI] <Release PartitionSnapshotHandle>");
    let handler = ptr as *mut FfiSnapshot;
    unsafe {
        Box::from_raw(handler);
    }
}

#[no_mangle]
pub extern fn ReleaseErrorHandle(ptr: ErrorHandle) {
    println!("[Rust FFI] <Release ErrorHandle>");
    let handler = ptr as *mut Error;
    unsafe {
        Box::from_raw(handler);
    }
}

#[no_mangle]
pub extern fn ReleaseVertexHandle(ptr: VertexHandle) {
    println!("[Rust FFI] <Release VertexHandle>");
    let handler = ptr as *mut FfiVertex;
    unsafe {
        Box::from_raw(handler);
    }
}

#[no_mangle]
pub extern fn ReleaseVertexIteratorHandle(ptr: VertexIteratorHandle) {
    println!("[Rust FFI] <Release VertexIteratorHandle>");
    let handler = ptr as *mut FfiVertexIterator;
    unsafe {
        Box::from_raw(handler);
    }
}

#[no_mangle]
pub extern fn ReleaseEdgeHandle(ptr: EdgeHandle) {
    println!("[Rust FFI] <Release EdgeHandle>");
    let handler = ptr as *mut FfiEdge;
    unsafe {
        Box::from_raw(handler);
    }
}

#[no_mangle]
pub extern fn ReleaseEdgeIteratorHandle(ptr: EdgeIteratorHandle) {
    println!("[Rust FFI] <Release EdgeIteratorHandle>");
    let handler = ptr as *mut FfiEdgeIterator;
    unsafe {
        Box::from_raw(handler);
    }
}

#[no_mangle]
pub extern fn ReleasePropertyHandle(ptr: PropertyHandle) {
    println!("[Rust FFI] <Release PropertyHandle>");
    let handler = ptr as *mut FfiProperty;
    unsafe {
        Box::from_raw(handler);
    }
}

#[no_mangle]
pub extern fn ReleasePropertyIteratorHandle(ptr: PropertyIteratorHandle) {
    println!("[Rust FFI] <Release PropertyIteratorHandle>");
    let handler = ptr as *mut FfiPropertyIterator;
    unsafe {
        Box::from_raw(handler);
    }
}

/// Internal functions

fn label_option(label_id : LabelId) -> Option<LabelId> {
    if label_id != LabelId::MAX {
        Some(label_id)
    } else {
        None
    }
}

fn edge_relation_option(edge_relation: &EdgeRelation) -> Option<&EdgeRelation> {
    if edge_relation.get_edge_label_id() != LabelId::MAX {
        Some(edge_relation)
    } else {
        None
    }
}
