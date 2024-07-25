// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use datafusion::functions_aggregate::all_default_aggregate_functions;
use datafusion_expr::AggregateExt;
use pyo3::{prelude::*, wrap_pyfunction};

use crate::common::data_type::NullTreatment;
use crate::context::PySessionContext;
use crate::errors::DataFusionError;
use crate::expr::conditional_expr::PyCaseBuilder;
use crate::expr::window::PyWindowFrame;
use crate::expr::PyExpr;
use datafusion::execution::FunctionRegistry;
use datafusion::functions;
use datafusion::functions_aggregate;
use datafusion_common::{Column, ScalarValue, TableReference};
use datafusion_expr::expr::Alias;
use datafusion_expr::{
    expr::{
        find_df_window_func, AggregateFunction, AggregateFunctionDefinition, Sort, WindowFunction,
    },
    lit, Expr, WindowFunctionDefinition,
};

#[pyfunction]
pub fn approx_distinct(expression: PyExpr) -> PyExpr {
    functions_aggregate::expr_fn::approx_distinct::approx_distinct(expression.expr).into()
}

#[pyfunction]
pub fn approx_median(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    // TODO: better builder pattern
    let expr = functions_aggregate::expr_fn::approx_median(expression.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn approx_percentile_cont(
    expression: PyExpr,
    percentile: PyExpr,
    distinct: bool,
) -> PyResult<PyExpr> {
    // TODO: better builder pattern
    let expr =
        functions_aggregate::expr_fn::approx_percentile_cont(expression.expr, percentile.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn approx_percentile_cont_with_weight(
    expression: PyExpr,
    weight: PyExpr,
    percentile: PyExpr,
    distinct: bool,
) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::approx_percentile_cont_with_weight(
        expression.expr,
        weight.expr,
        percentile.expr,
    );
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn avg(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::avg(expression.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn bit_and(expr_x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::bit_and(expr_x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn bit_or(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::bit_or(expression.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn bit_xor(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::bit_xor(expression.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn bool_and(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::bool_and(expression.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn bool_or(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::bool_or(expression.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn mean(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    // alias for avg
    avg(expression, distinct)
}

#[pyfunction]
pub fn corr(y: PyExpr, x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::corr(y.expr, x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn grouping(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::grouping(expression.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn sum(args: PyExpr) -> PyExpr {
    functions_aggregate::expr_fn::sum(args.expr).into()
}

#[pyfunction]
pub fn covar_samp(y: PyExpr, x: PyExpr) -> PyExpr {
    functions_aggregate::expr_fn::covar_samp(y.expr, x.expr).into()
}

#[pyfunction]
pub fn covar_pop(y: PyExpr, x: PyExpr) -> PyExpr {
    functions_aggregate::expr_fn::covar_pop(y.expr, x.expr).into()
}

#[pyfunction]
pub fn median(arg: PyExpr) -> PyExpr {
    functions_aggregate::expr_fn::median(arg.expr).into()
}

#[pyfunction]
pub fn covar(y: PyExpr, x: PyExpr) -> PyExpr {
    // alias for covar_samp
    covar_samp(y, x)
}

#[pyfunction]
pub fn stddev(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::stddev(expression.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn stddev_pop(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::stddev_pop(expression.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn var_samp(expression: PyExpr) -> PyExpr {
    functions_aggregate::expr_fn::var_sample(expression.expr).into()
}

#[pyfunction]
/// Alias for [`var_samp`]
pub fn var(y: PyExpr) -> PyExpr {
    var_samp(y)
}

#[pyfunction]
pub fn var_pop(expression: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::var_pop(expression.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn regr_avgx(expr_y: PyExpr, expr_x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::regr_avgx(expr_y.expr, expr_x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn regr_avgy(expr_y: PyExpr, expr_x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::regr_avgy(expr_y.expr, expr_x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn regr_count(expr_y: PyExpr, expr_x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::regr_count(expr_y.expr, expr_x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn regr_intercept(expr_y: PyExpr, expr_x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::regr_intercept(expr_y.expr, expr_x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn regr_r2(expr_y: PyExpr, expr_x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::regr_r2(expr_y.expr, expr_x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn regr_slope(expr_y: PyExpr, expr_x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::regr_slope(expr_y.expr, expr_x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn regr_sxx(expr_y: PyExpr, expr_x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::regr_sxx(expr_y.expr, expr_x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn regr_sxy(expr_y: PyExpr, expr_x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::regr_sxy(expr_y.expr, expr_x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
pub fn regr_syy(expr_y: PyExpr, expr_x: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::regr_syy(expr_y.expr, expr_x.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

#[pyfunction]
#[pyo3(signature = (expr, distinct = false, filter = None, order_by = None, null_treatment = None))]
pub fn first_value(
    expr: PyExpr,
    distinct: bool,
    filter: Option<PyExpr>,
    order_by: Option<Vec<PyExpr>>,
    null_treatment: Option<NullTreatment>,
) -> PyResult<PyExpr> {
    let order_by = order_by.map(|x| x.into_iter().map(|x| x.expr).collect::<Vec<_>>());

    // TODO: add `builder()` to `AggregateExt` to avoid this boilerplate
    let builder = functions_aggregate::expr_fn::first_value(expr.expr, order_by);

    let builder = if let Some(filter) = filter {
        let filter = filter.expr;
        builder.filter(filter).build()?
    } else {
        builder
    };

    let builder = if distinct {
        builder.distinct().build()?
    } else {
        builder
    };

    let builder = if let Some(null_treatment) = null_treatment {
        builder.null_treatment(null_treatment.into()).build()?
    } else {
        builder
    };

    Ok(builder.into())
}

#[pyfunction]
#[pyo3(signature = (expr, distinct = false, filter = None, order_by = None, null_treatment = None))]
pub fn last_value(
    expr: PyExpr,
    distinct: bool,
    filter: Option<PyExpr>,
    order_by: Option<Vec<PyExpr>>,
    null_treatment: Option<NullTreatment>,
) -> PyResult<PyExpr> {
    // TODO: add `builder()` to `AggregateExt` to avoid this boilerplate
    let builder = functions_aggregate::expr_fn::last_value(vec![expr.expr]);

    let builder = if distinct {
        builder.distinct().build()?
    } else {
        builder
    };

    let builder = if let Some(filter) = filter {
        let filter = filter.expr;
        builder.filter(filter).build()?
    } else {
        builder
    };

    let builder = if let Some(order_by) = order_by {
        let order_by = order_by.into_iter().map(|x| x.expr).collect::<Vec<_>>();
        builder.order_by(order_by).build()?
    } else {
        builder
    };

    let builder = if let Some(null_treatment) = null_treatment {
        builder.null_treatment(null_treatment.into()).build()?
    } else {
        builder
    };

    Ok(builder.into())
}

#[pyfunction]
fn in_list(expr: PyExpr, value: Vec<PyExpr>, negated: bool) -> PyExpr {
    datafusion_expr::in_list(
        expr.expr,
        value.into_iter().map(|x| x.expr).collect::<Vec<_>>(),
        negated,
    )
    .into()
}

#[pyfunction]
#[pyo3(signature = (*exprs))]
fn make_array(exprs: Vec<PyExpr>) -> PyExpr {
    datafusion_functions_array::expr_fn::make_array(exprs.into_iter().map(|x| x.into()).collect())
        .into()
}

#[pyfunction]
#[pyo3(signature = (*exprs))]
fn array(exprs: Vec<PyExpr>) -> PyExpr {
    // alias for make_array
    make_array(exprs)
}

#[pyfunction]
#[pyo3(signature = (*exprs))]
fn array_concat(exprs: Vec<PyExpr>) -> PyExpr {
    let exprs = exprs.into_iter().map(|x| x.into()).collect();
    datafusion_functions_array::expr_fn::array_concat(exprs).into()
}

#[pyfunction]
#[pyo3(signature = (*exprs))]
fn array_cat(exprs: Vec<PyExpr>) -> PyExpr {
    array_concat(exprs)
}

#[pyfunction]
#[pyo3(signature = (array, element, index = 1))]
fn array_position(array: PyExpr, element: PyExpr, index: Option<i64>) -> PyExpr {
    let index = ScalarValue::Int64(index);
    let index = Expr::Literal(index);
    datafusion_functions_array::expr_fn::array_position(array.into(), element.into(), index).into()
}

#[pyfunction]
#[pyo3(signature = (array, element, index = 1))]
fn array_indexof(array: PyExpr, element: PyExpr, index: Option<i64>) -> PyExpr {
    // alias of array_position
    array_position(array, element, index)
}

#[pyfunction]
#[pyo3(signature = (array, element, index = 1))]
fn list_position(array: PyExpr, element: PyExpr, index: Option<i64>) -> PyExpr {
    // alias of array_position
    array_position(array, element, index)
}

#[pyfunction]
#[pyo3(signature = (array, element, index = 1))]
fn list_indexof(array: PyExpr, element: PyExpr, index: Option<i64>) -> PyExpr {
    // alias of array_position
    array_position(array, element, index)
}

#[pyfunction]
#[pyo3(signature = (array, begin, end, stride = None))]
fn array_slice(array: PyExpr, begin: PyExpr, end: PyExpr, stride: Option<PyExpr>) -> PyExpr {
    datafusion_functions_array::expr_fn::array_slice(
        array.into(),
        begin.into(),
        end.into(),
        stride.map(Into::into),
    )
    .into()
}

#[pyfunction]
#[pyo3(signature = (array, begin, end, stride = None))]
fn list_slice(array: PyExpr, begin: PyExpr, end: PyExpr, stride: Option<PyExpr>) -> PyExpr {
    // alias of array_slice
    array_slice(array, begin, end, stride)
}

/// Computes a binary hash of the given data. type is the algorithm to use.
/// Standard algorithms are md5, sha224, sha256, sha384, sha512, blake2s, blake2b, and blake3.
// #[pyfunction(value, method)]
#[pyfunction]
#[pyo3(signature = (value, method))]
fn digest(value: PyExpr, method: PyExpr) -> PyExpr {
    PyExpr {
        expr: functions::expr_fn::digest(value.expr, method.expr),
    }
}

/// Concatenates the text representations of all the arguments.
/// NULL arguments are ignored.
#[pyfunction]
#[pyo3(signature = (*args))]
fn concat(args: Vec<PyExpr>) -> PyResult<PyExpr> {
    let args = args.into_iter().map(|e| e.expr).collect::<Vec<_>>();
    Ok(functions::string::expr_fn::concat(args).into())
}

/// Concatenates all but the first argument, with separators.
/// The first argument is used as the separator string, and should not be NULL.
/// Other NULL arguments are ignored.
#[pyfunction]
#[pyo3(signature = (sep, *args))]
fn concat_ws(sep: String, args: Vec<PyExpr>) -> PyResult<PyExpr> {
    let args = args.into_iter().map(|e| e.expr).collect::<Vec<_>>();
    Ok(functions::string::expr_fn::concat_ws(lit(sep), args).into())
}

#[pyfunction]
#[pyo3(signature = (values, regex, flags = None))]
fn regexp_like(values: PyExpr, regex: PyExpr, flags: Option<PyExpr>) -> PyResult<PyExpr> {
    Ok(functions::expr_fn::regexp_like(values.expr, regex.expr, flags.map(|x| x.expr)).into())
}

#[pyfunction]
#[pyo3(signature = (values, regex, flags = None))]
fn regexp_match(values: PyExpr, regex: PyExpr, flags: Option<PyExpr>) -> PyResult<PyExpr> {
    Ok(functions::expr_fn::regexp_match(values.expr, regex.expr, flags.map(|x| x.expr)).into())
}

#[pyfunction]
/// Replaces substring(s) matching a POSIX regular expression.
fn regexp_replace(
    string: PyExpr,
    pattern: PyExpr,
    replacement: PyExpr,
    flags: Option<PyExpr>,
) -> PyResult<PyExpr> {
    Ok(functions::expr_fn::regexp_replace(
        string.into(),
        pattern.into(),
        replacement.into(),
        flags.map(|x| x.expr),
    )
    .into())
}
/// Creates a new Sort Expr
#[pyfunction]
fn order_by(expr: PyExpr, asc: bool, nulls_first: bool) -> PyResult<PyExpr> {
    Ok(PyExpr {
        expr: datafusion_expr::Expr::Sort(Sort {
            expr: Box::new(expr.expr),
            asc,
            nulls_first,
        }),
    })
}

/// Creates a new Alias Expr
#[pyfunction]
fn alias(expr: PyExpr, name: &str) -> PyResult<PyExpr> {
    let relation: Option<TableReference> = None;
    Ok(PyExpr {
        expr: datafusion_expr::Expr::Alias(Alias::new(expr.expr, relation, name)),
    })
}

/// Create a column reference Expr
#[pyfunction]
fn col(name: &str) -> PyResult<PyExpr> {
    Ok(PyExpr {
        expr: datafusion_expr::Expr::Column(Column {
            relation: None,
            name: name.to_string(),
        }),
    })
}

// TODO: should we just expose this in python?
/// Create a COUNT(1) aggregate expression
#[pyfunction]
fn count_star() -> PyExpr {
    functions_aggregate::expr_fn::count(lit(1)).into()
}

/// Wrapper for [`functions_aggregate::expr_fn::count`]
/// Count the number of non-null values in the column
#[pyfunction]
fn count(expr: PyExpr, distinct: bool) -> PyResult<PyExpr> {
    let expr = functions_aggregate::expr_fn::count(expr.expr);
    if distinct {
        Ok(expr.distinct().build()?.into())
    } else {
        Ok(expr.into())
    }
}

/// Create a CASE WHEN statement with literal WHEN expressions for comparison to the base expression.
#[pyfunction]
fn case(expr: PyExpr) -> PyResult<PyCaseBuilder> {
    Ok(PyCaseBuilder {
        case_builder: datafusion_expr::case(expr.expr),
    })
}

/// Helper function to find the appropriate window function.
///
/// Search procedure:
/// 1) Search built in window functions, which are being deprecated.
/// 1) If a session context is provided:
///      1) search User Defined Aggregate Functions (UDAFs)
///      1) search registered window functions
///      1) search registered aggregate functions
/// 1) If no function has been found, search default aggregate functions.
///
/// NOTE: we search the built-ins first because the `UDAF` versions currently do not have the same behavior.
fn find_window_fn(name: &str, ctx: Option<PySessionContext>) -> PyResult<WindowFunctionDefinition> {
    // search built in window functions (soon to be deprecated)
    let df_window_func = find_df_window_func(name);
    if let Some(df_window_func) = df_window_func {
        return Ok(df_window_func);
    }

    if let Some(ctx) = ctx {
        // search UDAFs
        let udaf = ctx
            .ctx
            .udaf(name)
            .map(WindowFunctionDefinition::AggregateUDF)
            .ok();

        if let Some(udaf) = udaf {
            return Ok(udaf);
        }

        let session_state = ctx.ctx.state();

        // search registered window functions
        let window_fn = session_state
            .window_functions()
            .get(name)
            .map(|f| WindowFunctionDefinition::WindowUDF(f.clone()));

        if let Some(window_fn) = window_fn {
            return Ok(window_fn);
        }

        // search registered aggregate functions
        let agg_fn = session_state
            .aggregate_functions()
            .get(name)
            .map(|f| WindowFunctionDefinition::AggregateUDF(f.clone()));

        if let Some(agg_fn) = agg_fn {
            return Ok(agg_fn);
        }
    }

    // search default aggregate functions
    let agg_fn = all_default_aggregate_functions()
        .iter()
        .find(|v| v.name() == name || v.aliases().contains(&name.to_string()))
        .map(|f| WindowFunctionDefinition::AggregateUDF(f.clone()));

    if let Some(agg_fn) = agg_fn {
        return Ok(agg_fn);
    }

    Err(DataFusionError::Common(format!("window function `{name}` not found")).into())
}

/// Creates a new Window function expression
#[pyfunction]
fn window(
    name: &str,
    args: Vec<PyExpr>,
    partition_by: Option<Vec<PyExpr>>,
    order_by: Option<Vec<PyExpr>>,
    window_frame: Option<PyWindowFrame>,
    ctx: Option<PySessionContext>,
) -> PyResult<PyExpr> {
    let fun = find_window_fn(name, ctx)?;
    let window_frame = window_frame
        .unwrap_or_else(|| PyWindowFrame::new("rows", None, Some(0)).unwrap())
        .into();
    Ok(PyExpr {
        expr: datafusion_expr::Expr::WindowFunction(WindowFunction {
            fun,
            args: args.into_iter().map(|x| x.expr).collect::<Vec<_>>(),
            partition_by: partition_by
                .unwrap_or_default()
                .into_iter()
                .map(|x| x.expr)
                .collect::<Vec<_>>(),
            order_by: order_by
                .unwrap_or_default()
                .into_iter()
                .map(|x| x.expr)
                .collect::<Vec<_>>(),
            window_frame,
            null_treatment: None,
        }),
    })
}

macro_rules! aggregate_function {
    ($NAME: ident, $FUNC: ident) => {
        aggregate_function!($NAME, $FUNC, stringify!($NAME));
    };
    ($NAME: ident, $FUNC: ident, $DOC: expr) => {
        #[doc = $DOC]
        #[pyfunction]
        #[pyo3(signature = (*args, distinct=false))]
        fn $NAME(args: Vec<PyExpr>, distinct: bool) -> PyExpr {
            let expr = datafusion_expr::Expr::AggregateFunction(AggregateFunction {
                func_def: AggregateFunctionDefinition::BuiltIn(
                    datafusion_expr::aggregate_function::AggregateFunction::$FUNC,
                ),
                args: args.into_iter().map(|e| e.into()).collect(),
                distinct,
                filter: None,
                order_by: None,
                null_treatment: None,
            });
            expr.into()
        }
    };
}

/// Generates a [pyo3] wrapper for [datafusion::functions::expr_fn]
///
/// These functions have explicit named arguments.
macro_rules! expr_fn {
    ($NAME: ident) => {
        expr_fn!($NAME, $NAME, , stringify!($NAME));
    };
    ($NAME:ident, $($arg:ident)*) => {
        expr_fn!($NAME, $NAME, $($arg)*, stringify!($FUNC));
    };
    ($NAME:ident, $FUNC:ident, $($arg:ident)*) => {
        expr_fn!($NAME, $FUNC, $($arg)*, stringify!($FUNC));
    };
    ($NAME: ident, $DOC: expr) => {
        expr_fn!($NAME, $NAME, ,$DOC);
    };
    ($NAME: ident, $($arg:ident)*, $DOC: expr) => {
        expr_fn!($NAME, $NAME, $($arg)* ,$DOC);
    };
    ($NAME: ident, $FUNC: ident, $($arg:ident)*, $DOC: expr) => {
        #[doc = $DOC]
        #[pyfunction]
        fn $NAME($($arg: PyExpr),*) -> PyExpr {
            functions::expr_fn::$FUNC($($arg.into()),*).into()
        }
    };
}

/// Generates a [pyo3] wrapper for [datafusion::functions::expr_fn]
///
/// These functions take a single `Vec<PyExpr>` argument using `pyo3(signature = (*args))`.
macro_rules! expr_fn_vec {
    ($NAME: ident) => {
        expr_fn_vec!($NAME, $NAME, stringify!($NAME));
    };
    ($NAME: ident, $DOC: expr) => {
        expr_fn_vec!($NAME, $NAME, $DOC);
    };
    ($NAME: ident, $FUNC: ident, $DOC: expr) => {
        #[doc = $DOC]
        #[pyfunction]
        #[pyo3(signature = (*args))]
        fn $NAME(args: Vec<PyExpr>) -> PyExpr {
            let args = args.into_iter().map(|e| e.into()).collect::<Vec<_>>();
            functions::expr_fn::$FUNC(args).into()
        }
    };
}

/// Generates a [pyo3] wrapper for [datafusion_functions_array::expr_fn]
///
/// These functions have explicit named arguments.
macro_rules! array_fn {
    ($NAME: ident) => {
        array_fn!($NAME, $NAME, , stringify!($NAME));
    };
    ($NAME:ident,  $($arg:ident)*) => {
        array_fn!($NAME, $NAME, $($arg)*, stringify!($FUNC));
    };
    ($NAME: ident, $FUNC:ident, $($arg:ident)*) => {
        array_fn!($NAME, $FUNC, $($arg)*, stringify!($FUNC));
    };
    ($NAME: ident, $DOC: expr) => {
        array_fn!($NAME, $NAME, , $DOC);
    };
    ($NAME: ident, $FUNC:ident,  $($arg:ident)*, $DOC:expr) => {
        #[doc = $DOC]
        #[pyfunction]
        fn $NAME($($arg: PyExpr),*) -> PyExpr {
            datafusion_functions_array::expr_fn::$FUNC($($arg.into()),*).into()
        }
    };
}

expr_fn!(abs, num);
expr_fn!(acos, num);
expr_fn!(acosh, num);
expr_fn!(ascii, arg1, "Returns the numeric code of the first character of the argument. In UTF8 encoding, returns the Unicode code point of the character. In other multibyte encodings, the argument must be an ASCII character.");
expr_fn!(asin, num);
expr_fn!(asinh, num);
expr_fn!(atan, num);
expr_fn!(atanh, num);
expr_fn!(atan2, y x);
expr_fn!(
    bit_length,
    arg,
    "Returns number of bits in the string (8 times the octet_length)."
);
expr_fn_vec!(btrim, "Removes the longest string containing only characters in characters (a space by default) from the start and end of string.");
expr_fn!(cbrt, num);
expr_fn!(ceil, num);
expr_fn!(
    character_length,
    string,
    "Returns number of characters in the string."
);
expr_fn!(length, string);
expr_fn!(char_length, string);
expr_fn!(chr, arg, "Returns the character with the given code.");
expr_fn_vec!(coalesce);
expr_fn!(cos, num);
expr_fn!(cosh, num);
expr_fn!(cot, num);
expr_fn!(degrees, num);
expr_fn!(decode, input encoding);
expr_fn!(encode, input encoding);
expr_fn!(ends_with, string suffix, "Returns true if string ends with suffix.");
expr_fn!(exp, num);
expr_fn!(factorial, num);
expr_fn!(floor, num);
expr_fn!(gcd, x y);
expr_fn!(initcap, string, "Converts the first letter of each word to upper case and the rest to lower case. Words are sequences of alphanumeric characters separated by non-alphanumeric characters.");
expr_fn!(isnan, num);
expr_fn!(iszero, num);
expr_fn!(levenshtein, string1 string2);
expr_fn!(lcm, x y);
expr_fn!(left, string n, "Returns first n characters in the string, or when n is negative, returns all but last |n| characters.");
expr_fn!(ln, num);
expr_fn!(log, base num);
expr_fn!(log10, num);
expr_fn!(log2, num);
expr_fn!(lower, arg1, "Converts the string to all lower case");
expr_fn_vec!(lpad, "Extends the string to length length by prepending the characters fill (a space by default). If the string is already longer than length then it is truncated (on the right).");
expr_fn_vec!(ltrim, "Removes the longest string containing only characters in characters (a space by default) from the start of string.");
expr_fn!(
    md5,
    input_arg,
    "Computes the MD5 hash of the argument, with the result written in hexadecimal."
);
expr_fn!(
    nanvl,
    x y,
    "Returns x if x is not NaN otherwise returns y."
);
expr_fn!(nullif, arg_1 arg_2);
expr_fn!(octet_length, args, "Returns number of bytes in the string. Since this version of the function accepts type character directly, it will not strip trailing spaces.");
expr_fn_vec!(overlay);
expr_fn!(pi);
expr_fn!(power, base exponent);
expr_fn!(pow, power, base exponent);
expr_fn!(radians, num);
expr_fn!(repeat, string n, "Repeats string the specified number of times.");
expr_fn!(
    replace,
    string from to,
    "Replaces all occurrences in string of substring from with substring to."
);
expr_fn!(
    reverse,
    string,
    "Reverses the order of the characters in the string."
);
expr_fn!(right, string n, "Returns last n characters in the string, or when n is negative, returns all but first |n| characters.");
expr_fn_vec!(round);
expr_fn_vec!(rpad, "Extends the string to length length by appending the characters fill (a space by default). If the string is already longer than length then it is truncated.");
expr_fn_vec!(rtrim, "Removes the longest string containing only characters in characters (a space by default) from the end of string.");
expr_fn!(sha224, input_arg1);
expr_fn!(sha256, input_arg1);
expr_fn!(sha384, input_arg1);
expr_fn!(sha512, input_arg1);
expr_fn!(signum, num);
expr_fn!(sin, num);
expr_fn!(sinh, num);
expr_fn!(
    split_part,
    string delimiter index,
    "Splits string at occurrences of delimiter and returns the n'th field (counting from one)."
);
expr_fn!(sqrt, num);
expr_fn!(starts_with, string prefix, "Returns true if string starts with prefix.");
expr_fn!(strpos, string substring, "Returns starting index of specified substring within string, or zero if it's not present. (Same as position(substring in string), but note the reversed argument order.)");
expr_fn!(substr, string position);
expr_fn!(substr_index, string delimiter count);
expr_fn!(substring, string position length);
expr_fn!(find_in_set, string string_list);
expr_fn!(tan, num);
expr_fn!(tanh, num);
expr_fn!(
    to_hex,
    arg1,
    "Converts the number to its equivalent hexadecimal representation."
);
expr_fn!(now);
expr_fn_vec!(to_timestamp);
expr_fn_vec!(to_timestamp_millis);
expr_fn_vec!(to_timestamp_micros);
expr_fn_vec!(to_timestamp_seconds);
expr_fn_vec!(to_unixtime);
expr_fn!(current_date);
expr_fn!(current_time);
expr_fn!(date_part, part date);
expr_fn!(datepart, date_part, part date);
expr_fn!(date_trunc, part date);
expr_fn!(datetrunc, date_trunc, part date);
expr_fn!(date_bin, stride source origin);
expr_fn!(make_date, year month day);

expr_fn!(translate, string from to, "Replaces each character in string that matches a character in the from set with the corresponding character in the to set. If from is longer than to, occurrences of the extra characters in from are deleted.");
expr_fn_vec!(trim, "Removes the longest string containing only characters in characters (a space by default) from the start, end, or both ends (BOTH is the default) of string.");
expr_fn_vec!(trunc);
expr_fn!(upper, arg1, "Converts the string to all upper case.");
expr_fn!(uuid);
expr_fn_vec!(r#struct); // Use raw identifier since struct is a keyword
expr_fn_vec!(named_struct);
expr_fn!(from_unixtime, unixtime);
expr_fn!(arrow_typeof, arg_1);
expr_fn!(random);

// Array Functions
array_fn!(array_append, array element);
array_fn!(array_push_back, array_append, array element);
array_fn!(array_to_string, array delimiter);
array_fn!(array_join, array_to_string, array delimiter);
array_fn!(list_to_string, array_to_string, array delimiter);
array_fn!(list_join, array_to_string, array delimiter);
array_fn!(list_append, array_append, array element);
array_fn!(list_push_back, array_append, array element);
array_fn!(array_dims, array);
array_fn!(array_distinct, array);
array_fn!(list_distinct, array_distinct, array);
array_fn!(list_dims, array_dims, array);
array_fn!(array_element, array element);
array_fn!(array_extract, array_element, array element);
array_fn!(list_element, array_element, array element);
array_fn!(list_extract, array_element, array element);
array_fn!(array_length, array);
array_fn!(list_length, array_length, array);
array_fn!(array_has, first_array second_array);
array_fn!(array_has_all, first_array second_array);
array_fn!(array_has_any, first_array second_array);
array_fn!(array_positions, array_positions, array element);
array_fn!(list_positions, array_positions, array element);
array_fn!(array_ndims, array);
array_fn!(list_ndims, array_ndims, array);
array_fn!(array_prepend, element array);
array_fn!(array_push_front, array_prepend, element array);
array_fn!(list_prepend, array_prepend, element array);
array_fn!(list_push_front, array_prepend, element array);
array_fn!(array_pop_back, array);
array_fn!(array_pop_front, array);
array_fn!(array_remove, array element);
array_fn!(list_remove, array_remove, array element);
array_fn!(array_remove_n, array element max);
array_fn!(list_remove_n, array_remove_n, array element max);
array_fn!(array_remove_all, array element);
array_fn!(list_remove_all, array_remove_all, array element);
array_fn!(array_repeat, element count);
array_fn!(array_replace, array from to);
array_fn!(list_replace, array_replace, array from to);
array_fn!(array_replace_n, array from to max);
array_fn!(list_replace_n, array_replace_n, array from to max);
array_fn!(array_replace_all, array from to);
array_fn!(list_replace_all, array_replace_all, array from to);
array_fn!(array_sort, array desc null_first);
array_fn!(list_sort, array_sort, array desc null_first);
array_fn!(array_intersect, first_array second_array);
array_fn!(list_intersect, array_intersect, first_array second_array);
array_fn!(array_union, array1 array2);
array_fn!(list_union, array_union, array1 array2);
array_fn!(array_except, first_array second_array);
array_fn!(list_except, array_except, first_array second_array);
array_fn!(array_resize, array size value);
array_fn!(list_resize, array_resize, array size value);
array_fn!(flatten, array);
array_fn!(range, start stop step);

aggregate_function!(array_agg, ArrayAgg);
aggregate_function!(max, Max);
aggregate_function!(min, Min);

pub(crate) fn init_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(abs))?;
    m.add_wrapped(wrap_pyfunction!(acos))?;
    m.add_wrapped(wrap_pyfunction!(acosh))?;
    m.add_wrapped(wrap_pyfunction!(approx_distinct))?;
    m.add_wrapped(wrap_pyfunction!(alias))?;
    m.add_wrapped(wrap_pyfunction!(approx_median))?;
    m.add_wrapped(wrap_pyfunction!(approx_percentile_cont))?;
    m.add_wrapped(wrap_pyfunction!(approx_percentile_cont_with_weight))?;
    m.add_wrapped(wrap_pyfunction!(array))?;
    m.add_wrapped(wrap_pyfunction!(range))?;
    m.add_wrapped(wrap_pyfunction!(array_agg))?;
    m.add_wrapped(wrap_pyfunction!(arrow_typeof))?;
    m.add_wrapped(wrap_pyfunction!(ascii))?;
    m.add_wrapped(wrap_pyfunction!(asin))?;
    m.add_wrapped(wrap_pyfunction!(asinh))?;
    m.add_wrapped(wrap_pyfunction!(atan))?;
    m.add_wrapped(wrap_pyfunction!(atanh))?;
    m.add_wrapped(wrap_pyfunction!(atan2))?;
    m.add_wrapped(wrap_pyfunction!(avg))?;
    m.add_wrapped(wrap_pyfunction!(bit_length))?;
    m.add_wrapped(wrap_pyfunction!(btrim))?;
    m.add_wrapped(wrap_pyfunction!(cbrt))?;
    m.add_wrapped(wrap_pyfunction!(ceil))?;
    m.add_wrapped(wrap_pyfunction!(character_length))?;
    m.add_wrapped(wrap_pyfunction!(chr))?;
    m.add_wrapped(wrap_pyfunction!(char_length))?;
    m.add_wrapped(wrap_pyfunction!(coalesce))?;
    m.add_wrapped(wrap_pyfunction!(case))?;
    m.add_wrapped(wrap_pyfunction!(col))?;
    m.add_wrapped(wrap_pyfunction!(concat_ws))?;
    m.add_wrapped(wrap_pyfunction!(concat))?;
    m.add_wrapped(wrap_pyfunction!(corr))?;
    m.add_wrapped(wrap_pyfunction!(cos))?;
    m.add_wrapped(wrap_pyfunction!(cosh))?;
    m.add_wrapped(wrap_pyfunction!(cot))?;
    m.add_wrapped(wrap_pyfunction!(count))?;
    m.add_wrapped(wrap_pyfunction!(count_star))?;
    m.add_wrapped(wrap_pyfunction!(covar))?;
    m.add_wrapped(wrap_pyfunction!(covar_pop))?;
    m.add_wrapped(wrap_pyfunction!(covar_samp))?;
    m.add_wrapped(wrap_pyfunction!(current_date))?;
    m.add_wrapped(wrap_pyfunction!(current_time))?;
    m.add_wrapped(wrap_pyfunction!(degrees))?;
    m.add_wrapped(wrap_pyfunction!(date_bin))?;
    m.add_wrapped(wrap_pyfunction!(datepart))?;
    m.add_wrapped(wrap_pyfunction!(date_part))?;
    m.add_wrapped(wrap_pyfunction!(datetrunc))?;
    m.add_wrapped(wrap_pyfunction!(date_trunc))?;
    m.add_wrapped(wrap_pyfunction!(make_date))?;
    m.add_wrapped(wrap_pyfunction!(digest))?;
    m.add_wrapped(wrap_pyfunction!(ends_with))?;
    m.add_wrapped(wrap_pyfunction!(exp))?;
    m.add_wrapped(wrap_pyfunction!(factorial))?;
    m.add_wrapped(wrap_pyfunction!(floor))?;
    m.add_wrapped(wrap_pyfunction!(from_unixtime))?;
    m.add_wrapped(wrap_pyfunction!(gcd))?;
    m.add_wrapped(wrap_pyfunction!(grouping))?;
    m.add_wrapped(wrap_pyfunction!(in_list))?;
    m.add_wrapped(wrap_pyfunction!(initcap))?;
    m.add_wrapped(wrap_pyfunction!(isnan))?;
    m.add_wrapped(wrap_pyfunction!(iszero))?;
    m.add_wrapped(wrap_pyfunction!(levenshtein))?;
    m.add_wrapped(wrap_pyfunction!(lcm))?;
    m.add_wrapped(wrap_pyfunction!(left))?;
    m.add_wrapped(wrap_pyfunction!(length))?;
    m.add_wrapped(wrap_pyfunction!(ln))?;
    m.add_wrapped(wrap_pyfunction!(log))?;
    m.add_wrapped(wrap_pyfunction!(log10))?;
    m.add_wrapped(wrap_pyfunction!(log2))?;
    m.add_wrapped(wrap_pyfunction!(lower))?;
    m.add_wrapped(wrap_pyfunction!(lpad))?;
    m.add_wrapped(wrap_pyfunction!(ltrim))?;
    m.add_wrapped(wrap_pyfunction!(max))?;
    m.add_wrapped(wrap_pyfunction!(make_array))?;
    m.add_wrapped(wrap_pyfunction!(md5))?;
    m.add_wrapped(wrap_pyfunction!(mean))?;
    m.add_wrapped(wrap_pyfunction!(median))?;
    m.add_wrapped(wrap_pyfunction!(min))?;
    m.add_wrapped(wrap_pyfunction!(named_struct))?;
    m.add_wrapped(wrap_pyfunction!(nanvl))?;
    m.add_wrapped(wrap_pyfunction!(now))?;
    m.add_wrapped(wrap_pyfunction!(nullif))?;
    m.add_wrapped(wrap_pyfunction!(octet_length))?;
    m.add_wrapped(wrap_pyfunction!(order_by))?;
    m.add_wrapped(wrap_pyfunction!(overlay))?;
    m.add_wrapped(wrap_pyfunction!(pi))?;
    m.add_wrapped(wrap_pyfunction!(power))?;
    m.add_wrapped(wrap_pyfunction!(pow))?;
    m.add_wrapped(wrap_pyfunction!(radians))?;
    m.add_wrapped(wrap_pyfunction!(random))?;
    m.add_wrapped(wrap_pyfunction!(regexp_like))?;
    m.add_wrapped(wrap_pyfunction!(regexp_match))?;
    m.add_wrapped(wrap_pyfunction!(regexp_replace))?;
    m.add_wrapped(wrap_pyfunction!(repeat))?;
    m.add_wrapped(wrap_pyfunction!(replace))?;
    m.add_wrapped(wrap_pyfunction!(reverse))?;
    m.add_wrapped(wrap_pyfunction!(right))?;
    m.add_wrapped(wrap_pyfunction!(round))?;
    m.add_wrapped(wrap_pyfunction!(rpad))?;
    m.add_wrapped(wrap_pyfunction!(rtrim))?;
    m.add_wrapped(wrap_pyfunction!(sha224))?;
    m.add_wrapped(wrap_pyfunction!(sha256))?;
    m.add_wrapped(wrap_pyfunction!(sha384))?;
    m.add_wrapped(wrap_pyfunction!(sha512))?;
    m.add_wrapped(wrap_pyfunction!(signum))?;
    m.add_wrapped(wrap_pyfunction!(sin))?;
    m.add_wrapped(wrap_pyfunction!(sinh))?;
    m.add_wrapped(wrap_pyfunction!(split_part))?;
    m.add_wrapped(wrap_pyfunction!(sqrt))?;
    m.add_wrapped(wrap_pyfunction!(starts_with))?;
    m.add_wrapped(wrap_pyfunction!(stddev))?;
    m.add_wrapped(wrap_pyfunction!(stddev_pop))?;
    m.add_wrapped(wrap_pyfunction!(strpos))?;
    m.add_wrapped(wrap_pyfunction!(r#struct))?; // Use raw identifier since struct is a keyword
    m.add_wrapped(wrap_pyfunction!(substr))?;
    m.add_wrapped(wrap_pyfunction!(substr_index))?;
    m.add_wrapped(wrap_pyfunction!(substring))?;
    m.add_wrapped(wrap_pyfunction!(find_in_set))?;
    m.add_wrapped(wrap_pyfunction!(sum))?;
    m.add_wrapped(wrap_pyfunction!(tan))?;
    m.add_wrapped(wrap_pyfunction!(tanh))?;
    m.add_wrapped(wrap_pyfunction!(to_hex))?;
    m.add_wrapped(wrap_pyfunction!(to_timestamp))?;
    m.add_wrapped(wrap_pyfunction!(to_timestamp_millis))?;
    m.add_wrapped(wrap_pyfunction!(to_timestamp_micros))?;
    m.add_wrapped(wrap_pyfunction!(to_timestamp_seconds))?;
    m.add_wrapped(wrap_pyfunction!(to_unixtime))?;
    m.add_wrapped(wrap_pyfunction!(translate))?;
    m.add_wrapped(wrap_pyfunction!(trim))?;
    m.add_wrapped(wrap_pyfunction!(trunc))?;
    m.add_wrapped(wrap_pyfunction!(upper))?;
    m.add_wrapped(wrap_pyfunction!(self::uuid))?; // Use self to avoid name collision
    m.add_wrapped(wrap_pyfunction!(var))?;
    m.add_wrapped(wrap_pyfunction!(var_pop))?;
    m.add_wrapped(wrap_pyfunction!(var_samp))?;
    m.add_wrapped(wrap_pyfunction!(window))?;
    m.add_wrapped(wrap_pyfunction!(regr_avgx))?;
    m.add_wrapped(wrap_pyfunction!(regr_avgy))?;
    m.add_wrapped(wrap_pyfunction!(regr_count))?;
    m.add_wrapped(wrap_pyfunction!(regr_intercept))?;
    m.add_wrapped(wrap_pyfunction!(regr_r2))?;
    m.add_wrapped(wrap_pyfunction!(regr_slope))?;
    m.add_wrapped(wrap_pyfunction!(regr_sxx))?;
    m.add_wrapped(wrap_pyfunction!(regr_sxy))?;
    m.add_wrapped(wrap_pyfunction!(regr_syy))?;
    m.add_wrapped(wrap_pyfunction!(first_value))?;
    m.add_wrapped(wrap_pyfunction!(last_value))?;
    m.add_wrapped(wrap_pyfunction!(bit_and))?;
    m.add_wrapped(wrap_pyfunction!(bit_or))?;
    m.add_wrapped(wrap_pyfunction!(bit_xor))?;
    m.add_wrapped(wrap_pyfunction!(bool_and))?;
    m.add_wrapped(wrap_pyfunction!(bool_or))?;

    //Binary String Functions
    m.add_wrapped(wrap_pyfunction!(encode))?;
    m.add_wrapped(wrap_pyfunction!(decode))?;

    // Array Functions
    m.add_wrapped(wrap_pyfunction!(array_append))?;
    m.add_wrapped(wrap_pyfunction!(array_push_back))?;
    m.add_wrapped(wrap_pyfunction!(list_append))?;
    m.add_wrapped(wrap_pyfunction!(list_push_back))?;
    m.add_wrapped(wrap_pyfunction!(array_concat))?;
    m.add_wrapped(wrap_pyfunction!(array_cat))?;
    m.add_wrapped(wrap_pyfunction!(array_dims))?;
    m.add_wrapped(wrap_pyfunction!(array_distinct))?;
    m.add_wrapped(wrap_pyfunction!(list_distinct))?;
    m.add_wrapped(wrap_pyfunction!(list_dims))?;
    m.add_wrapped(wrap_pyfunction!(array_element))?;
    m.add_wrapped(wrap_pyfunction!(array_extract))?;
    m.add_wrapped(wrap_pyfunction!(list_element))?;
    m.add_wrapped(wrap_pyfunction!(list_extract))?;
    m.add_wrapped(wrap_pyfunction!(array_length))?;
    m.add_wrapped(wrap_pyfunction!(list_length))?;
    m.add_wrapped(wrap_pyfunction!(array_has))?;
    m.add_wrapped(wrap_pyfunction!(array_has_all))?;
    m.add_wrapped(wrap_pyfunction!(array_has_any))?;
    m.add_wrapped(wrap_pyfunction!(array_position))?;
    m.add_wrapped(wrap_pyfunction!(array_indexof))?;
    m.add_wrapped(wrap_pyfunction!(list_position))?;
    m.add_wrapped(wrap_pyfunction!(list_indexof))?;
    m.add_wrapped(wrap_pyfunction!(array_positions))?;
    m.add_wrapped(wrap_pyfunction!(list_positions))?;
    m.add_wrapped(wrap_pyfunction!(array_to_string))?;
    m.add_wrapped(wrap_pyfunction!(array_intersect))?;
    m.add_wrapped(wrap_pyfunction!(list_intersect))?;
    m.add_wrapped(wrap_pyfunction!(array_union))?;
    m.add_wrapped(wrap_pyfunction!(list_union))?;
    m.add_wrapped(wrap_pyfunction!(array_except))?;
    m.add_wrapped(wrap_pyfunction!(list_except))?;
    m.add_wrapped(wrap_pyfunction!(array_resize))?;
    m.add_wrapped(wrap_pyfunction!(list_resize))?;
    m.add_wrapped(wrap_pyfunction!(array_join))?;
    m.add_wrapped(wrap_pyfunction!(list_to_string))?;
    m.add_wrapped(wrap_pyfunction!(list_join))?;
    m.add_wrapped(wrap_pyfunction!(array_ndims))?;
    m.add_wrapped(wrap_pyfunction!(list_ndims))?;
    m.add_wrapped(wrap_pyfunction!(array_prepend))?;
    m.add_wrapped(wrap_pyfunction!(array_push_front))?;
    m.add_wrapped(wrap_pyfunction!(list_prepend))?;
    m.add_wrapped(wrap_pyfunction!(list_push_front))?;
    m.add_wrapped(wrap_pyfunction!(array_pop_back))?;
    m.add_wrapped(wrap_pyfunction!(array_pop_front))?;
    m.add_wrapped(wrap_pyfunction!(array_remove))?;
    m.add_wrapped(wrap_pyfunction!(list_remove))?;
    m.add_wrapped(wrap_pyfunction!(array_remove_n))?;
    m.add_wrapped(wrap_pyfunction!(list_remove_n))?;
    m.add_wrapped(wrap_pyfunction!(array_remove_all))?;
    m.add_wrapped(wrap_pyfunction!(list_remove_all))?;
    m.add_wrapped(wrap_pyfunction!(array_repeat))?;
    m.add_wrapped(wrap_pyfunction!(array_replace))?;
    m.add_wrapped(wrap_pyfunction!(list_replace))?;
    m.add_wrapped(wrap_pyfunction!(array_replace_n))?;
    m.add_wrapped(wrap_pyfunction!(list_replace_n))?;
    m.add_wrapped(wrap_pyfunction!(array_replace_all))?;
    m.add_wrapped(wrap_pyfunction!(list_replace_all))?;
    m.add_wrapped(wrap_pyfunction!(array_sort))?;
    m.add_wrapped(wrap_pyfunction!(list_sort))?;
    m.add_wrapped(wrap_pyfunction!(array_slice))?;
    m.add_wrapped(wrap_pyfunction!(list_slice))?;
    m.add_wrapped(wrap_pyfunction!(flatten))?;

    Ok(())
}
