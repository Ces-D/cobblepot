use crate::shared::{create_budget_recurrence, validate_rrule};
use cobblepot_data_store::{RecurrenceRule, UnixTimestamp, schema};
use diesel::{
    AsChangeset, Connection, ExpressionMethods, Identifiable, QueryResult, SqliteConnection,
};

#[derive(Identifiable, AsChangeset)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = schema::budget)]
pub struct UpdateBudget {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub anticipated_amount: Option<f32>,
}

#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = schema::budget_recurrence)]
pub struct UpdateBudgetRecurrence {
    pub id: i32,
    pub dt_start: Option<UnixTimestamp>,
    pub recurrence_rule: Option<RecurrenceRule>,
}

#[derive(Identifiable, AsChangeset, Debug)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = schema::budget_item)]
pub struct UpdateBudgetItem {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub amount: Option<f32>,
    pub budget_id: Option<i32>,
}

pub fn update_budget(
    mut conn: SqliteConnection,
    id: i32,
    name: Option<String>,
    description: Option<String>,
    anticipated_amount: Option<f32>,
    r_start: Option<UnixTimestamp>,
    r_rule: Option<RecurrenceRule>,
) -> QueryResult<(i32, String)> {
    use cobblepot_data_store::Budget;
    use diesel::{QueryDsl, RunQueryDsl};

    conn.transaction(|conn| {
        // Update budget if at least one field is provided, otherwise just fetch
        let budget_updated = if name.is_some()
            || description.is_some()
            || anticipated_amount.is_some()
        {
            let budget_changeset = UpdateBudget {
                id,
                name,
                description,
                anticipated_amount,
            };
            diesel::update(&budget_changeset).set(&budget_changeset).get_result::<Budget>(conn)?
        } else {
            schema::budget::table.find(id).first(conn)?
        };

        // Update budget_recurrence if it exists and recurrence fields are provided
        if let Some(rec_id) = budget_updated.budget_recurrence_id {
            if r_start.is_some() || r_rule.is_some() {
                use cobblepot_data_store::BudgetRecurrence;

                // Fetch current recurrence to get existing values for validation
                let current_recurrence: BudgetRecurrence =
                    schema::budget_recurrence::table.find(rec_id).first(conn)?;

                // Get the dt_start to validate against (new one or existing)
                let dt_start_for_validation =
                    r_start.clone().unwrap_or_else(|| current_recurrence.dt_start.clone());

                // Get the rrule to validate (new one or existing)
                let mut rrule_to_validate =
                    r_rule.clone().unwrap_or_else(|| current_recurrence.recurrence_rule.clone());

                // Validate the rrule against the dt_start
                let validated = validate_rrule(&dt_start_for_validation, &mut rrule_to_validate);

                let recurrence_changeset = UpdateBudgetRecurrence {
                    id: rec_id,
                    dt_start: r_start,
                    recurrence_rule: if r_rule.is_some() {
                        Some(validated)
                    } else {
                        None
                    },
                };
                diesel::update(&recurrence_changeset).set(&recurrence_changeset).execute(conn)?;
            }
        } else if let Some((dt_start, rrule)) = r_start.zip(r_rule) {
            // Create new recurrence
            let rec_id =
                create_budget_recurrence(conn, dt_start, rrule, Some(budget_updated.id), None)?;

            // Update budget to point to the new recurrence
            diesel::update(schema::budget::table.filter(schema::budget::id.eq(budget_updated.id)))
                .set(schema::budget::budget_recurrence_id.eq(rec_id))
                .execute(conn)?;
        }

        Ok((budget_updated.id, budget_updated.name))
    })
}

pub fn update_budget_item(
    mut conn: SqliteConnection,
    id: i32,
    budget_id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    amount: Option<f32>,
    r_start: Option<UnixTimestamp>,
    r_rule: Option<RecurrenceRule>,
) -> QueryResult<(i32, String)> {
    use cobblepot_data_store::BudgetItem;
    use diesel::{QueryDsl, RunQueryDsl};

    conn.transaction(|conn| {
        // Update budget_item if at least one field is provided, otherwise just fetch
        let budget_item_updated =
            if name.is_some() || description.is_some() || amount.is_some() || budget_id.is_some() {
                let budget_item_changeset = UpdateBudgetItem {
                    id,
                    name,
                    description,
                    amount,
                    budget_id,
                };
                diesel::update(&budget_item_changeset)
                    .set(&budget_item_changeset)
                    .get_result::<BudgetItem>(conn)?
            } else {
                schema::budget_item::table.find(id).first(conn)?
            };

        // Update budget_recurrence if it exists and recurrence fields are provided
        if let Some(rec_id) = budget_item_updated.budget_recurrence_id {
            if r_start.is_some() || r_rule.is_some() {
                use cobblepot_data_store::BudgetRecurrence;

                // Fetch current recurrence to get existing values for validation
                let current_recurrence: BudgetRecurrence =
                    schema::budget_recurrence::table.find(rec_id).first(conn)?;

                // Get the dt_start to validate against (new one or existing)
                let dt_start_for_validation =
                    r_start.clone().unwrap_or_else(|| current_recurrence.dt_start.clone());

                // Get the rrule to validate (new one or existing)
                let mut rrule_to_validate =
                    r_rule.clone().unwrap_or_else(|| current_recurrence.recurrence_rule.clone());

                // Validate the rrule against the dt_start
                let validated = validate_rrule(&dt_start_for_validation, &mut rrule_to_validate);

                let recurrence_changeset = UpdateBudgetRecurrence {
                    id: rec_id,
                    dt_start: r_start,
                    recurrence_rule: if r_rule.is_some() {
                        Some(validated)
                    } else {
                        None
                    },
                };
                diesel::update(&recurrence_changeset).set(&recurrence_changeset).execute(conn)?;
            }
        } else if let Some((dt_start, rrule)) = r_start.zip(r_rule) {
            // Create new recurrence
            let rec_id = create_budget_recurrence(
                conn,
                dt_start,
                rrule,
                None,
                Some(budget_item_updated.id),
            )?;

            // Update budget_item to point to the new recurrence
            diesel::update(
                schema::budget_item::table
                    .filter(schema::budget_item::id.eq(budget_item_updated.id)),
            )
            .set(schema::budget_item::budget_recurrence_id.eq(rec_id))
            .execute(conn)?;
        }

        Ok((budget_item_updated.id, budget_item_updated.name))
    })
}
