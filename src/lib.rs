#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_borrowck;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_macros;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_mir_build;
extern crate rustc_mir_dataflow;
extern crate rustc_mir_transform;
extern crate rustc_resolve;
extern crate rustc_serialize;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;
extern crate rustc_typeck;

pub use rustc_ast::ast::{MacArgs, MacArgsEq};
pub use rustc_ast::Mutability::*;
pub use rustc_ast::{AttrItem, AttrKind, Attribute};
pub use rustc_ast::{LitIntType, LitKind};
pub use rustc_borrowck::borrow_set::{BorrowSet, TwoPhaseActivation};
pub use rustc_data_structures::captures::Captures;
pub use rustc_data_structures::fx::{FxHashMap, FxIndexSet};
pub use rustc_data_structures::graph::WithSuccessors;
pub use rustc_data_structures::owning_ref::OwningRef;
pub use rustc_data_structures::rustc_erase_owner;
pub use rustc_data_structures::sync::{Lrc, MetadataRef};
pub use rustc_driver::RunCompiler;
pub use rustc_driver::{Callbacks, Compilation};
pub use rustc_errors::ErrorGuaranteed;
pub use rustc_errors::{DiagnosticBuilder, DiagnosticId};
pub use rustc_hir::def::DefKind;
pub use rustc_hir::def_id::{
    CrateNum, DefId, DefIndex, DefPathHash, LocalDefId, StableCrateId, LOCAL_CRATE,
};
pub use rustc_hir::itemlikevisit::ItemLikeVisitor;
pub use rustc_hir::HirId;
pub use rustc_hir::Mutability::*;
pub use rustc_hir::{ForeignItem, ImplItem, Item, TraitItem};
pub use rustc_index::bit_set::BitSet;
pub use rustc_index::vec::Idx;
pub use rustc_index::vec::IndexVec;
pub use rustc_infer::infer::NllRegionVariableOrigin;
pub use rustc_interface::interface::try_print_query_stack;
pub use rustc_interface::{interface::Compiler, Config, Queries};
pub use rustc_macros::{Decodable, Encodable, TyDecodable, TyEncodable, TypeFoldable};
pub use rustc_metadata::creader::CStore;
pub use rustc_middle::hir::map::Map;
pub use rustc_middle::hir::nested_filter::OnlyBodies;
pub use rustc_middle::implement_ty_decoder;

pub mod mir {
    pub use rustc_middle::mir;  // TODO: Remove
    pub use rustc_middle::mir::interpret::{AllocId, AllocRange, ConstValue};
    pub use rustc_middle::mir::traversal::preorder;
    pub use rustc_middle::mir::visit::{
        MutVisitor, MutatingUseContext, NonMutatingUseContext, NonUseContext, PlaceContext,
        TyContext, Visitor,
    };
    pub use rustc_middle::mir::VarDebugInfoContents;
    pub use rustc_middle::mir::{AggregateKind, Place, Rvalue, StatementKind, TerminatorKind};
    pub use rustc_middle::mir::{BasicBlock, BasicBlockData, Body, Promoted};
    pub use rustc_middle::mir::{Field, Local, Location};
}

pub mod thir {
    pub use rustc_middle::thir; // TODO: Remove
    pub use rustc_middle::thir::visit::Visitor;
    pub use rustc_middle::thir::{Expr, ExprKind, Thir};
}

pub mod ty {
    pub use rustc_middle::ty;  // TODO: Remove
    pub use rustc_middle::ty::codec::TyDecoder;
    pub use rustc_middle::ty::codec::TyEncoder;
    pub use rustc_middle::ty::fold::TypeFoldable;
    pub use rustc_middle::ty::subst::{InternalSubsts, Subst, SubstsRef};
    pub use rustc_middle::ty::AssocItemContainer;
    pub use rustc_middle::ty::ClosureKind;
    pub use rustc_middle::ty::DefIdTree;
    pub use rustc_middle::ty::GenericParamDef;
    pub use rustc_middle::ty::GenericParamDefKind;
    pub use rustc_middle::ty::PredicateKind;
    pub use rustc_middle::ty::ProjectionTy;
    pub use rustc_middle::ty::ReErased;
    pub use rustc_middle::ty::TraitRef;
    pub use rustc_middle::ty::Ty;
    pub use rustc_middle::ty::TyCtxt;
    pub use rustc_middle::ty::TyKind;
    pub use rustc_middle::ty::VariantDef;
    pub use rustc_middle::ty::Visibility;
    pub use rustc_middle::ty::WithOptConstParam;
    pub use rustc_middle::ty::{AdtDef, UpvarSubsts};
    pub use rustc_middle::ty::{AssocItem, Binder};
    pub use rustc_middle::ty::{ClosureSubsts, FieldDef};
    pub use rustc_middle::ty::{FloatTy, IntTy, UintTy};
    pub use rustc_middle::ty::{ParamEnv, Predicate};
}

pub use rustc_mir_dataflow::impls::{MaybeInitializedLocals, MaybeLiveLocals};
pub use rustc_mir_dataflow::move_paths::MoveData;
pub use rustc_mir_dataflow::Analysis;
pub use rustc_mir_dataflow::{AnalysisDomain, GenKill, GenKillAnalysis};
pub use rustc_mir_dataflow::{Results, ResultsCursor};

pub use rustc_mir_transform::{remove_false_edges::*, simplify::*};

pub use rustc_serialize::opaque;  // TODO: Remove
pub use rustc_serialize::{Decodable, Decoder};
pub use rustc_serialize::{Encodable, Encoder};

pub use rustc_session::cstore::CrateStore;
pub use rustc_session::Session;

pub use rustc_span::symbol::kw;  // TODO: Remove
pub use rustc_span::{Span, Symbol, DUMMY_SP};

pub use rustc_target::abi::Size;
pub use rustc_target::abi::VariantIdx;

pub use rustc_trait_selection::infer::InferCtxt;
pub use rustc_trait_selection::infer::TyCtxtInferExt;
pub use rustc_trait_selection::traits::error_reporting::InferCtxtExt;
pub use rustc_trait_selection::traits::FulfillmentContext;
pub use rustc_trait_selection::traits::ImplSource;
