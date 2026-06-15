use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

fn condition_check(row: &Row, dataset: &Dataset, condition: &Condition) -> bool {
    match condition {
        Condition::Equal(col_name, target_val) => {
            let col_index = dataset.column_index(col_name);
            row.get_value(col_index) == target_val
        }
        Condition::Not(inner) => {
            !condition_check(row,dataset,inner)
        }
        Condition::And(left,right) => {
            condition_check(row, dataset, left) && condition_check(row,dataset,right)
        }
        Condition::Or(left, right) => {
            condition_check(row, dataset, left) || condition_check(row, dataset, right)
        }

    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut result = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() {
        if condition_check(row, dataset, filter) {
            result.add_row(row.clone());
        }
    }
    result
    
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let col_index = dataset.column_index(group_by_column);
    let columns = dataset.columns().clone();
    let mut result: HashMap<Value, Dataset> = HashMap::new();
    for row in dataset.into_iter() {
        let key = row.get_value(col_index).clone();
        let bucket = result.entry(key).or_insert_with(|| Dataset::new(columns.clone()));
        bucket.add_row(row);
    }
    result

}

fn get_integer_values(dataset: &Dataset, col_name: &String) -> Vec<i32> {
    let col_index = dataset.column_index(col_name);
    let mut values = Vec::new();
    for row in dataset.iter() {
        match row.get_value(col_index) {
            Value::Integer(n) => values.push(*n),
            Value::String(_) => panic!("Not an integer column"),
        }
    }
    values
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result = HashMap::new();
    for (key, group) in dataset {
        let aggregate = match aggregation {
            Aggregation::Count(col_name) => {
                let col_index = group.column_index(col_name);
                Value::Integer(group.iter().count() as i32)
            }
            Aggregation::Sum(col_name) => {
                let values = get_integer_values(&group, col_name);
                let total: i32 = values.iter().sum();
                Value::Integer(total)
            }
            Aggregation::Average(col_name)=> {
                let values = get_integer_values(&group, col_name);
                let total: i32 = values.iter().sum();
                let avg = total/ values.len() as i32;
                Value::Integer(avg)
            }
        };
        result.insert(key, aggregate);
    }
    result
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}