//! Schema builder API
//!

use table::*;

pub struct Schema {
    schema: Option<String>,
    jobs: Vec<(Table, Box<Fn(&mut Table)>)>,
}

impl Schema {
    pub fn new() -> Schema {
        return Schema::make(None);
    }

    pub fn name(s: &str) -> Schema {
        return Schema::make(Some(s));
    }

    /// Add a table to the schema with a callback
    ///
    /// The callback is provided with a mutable table that fields
    /// can be worked on.
    ///
    /// This function is lazy
    pub fn create_table<F: 'static>(mut self, name: &str, cb: F) -> Schema
    where
        F: Fn(&mut Table),
    {
        let t = Table { name: String::from(name), items: Vec::new() };
        self.jobs.push((t, Box::new(cb)));
        return self;
    }

    /// Executes all hooks and does magic
    /// 
    /// Needs to be mutable because it morphs the hooks
    pub fn exec(&mut self) -> String {
        let mut s = String::new();

        for pair in &mut self.jobs {
            let (mut table, hook) = (&mut pair.0, &pair.1);
            let schema: &String = self.schema.as_ref().unwrap();
            hook(&mut table);
            let table_name: &String = &table.name;

            s.push_str("create table ");
            s.push_str(&format!("\"{}\".\"{}\"", schema, table_name));

            s.push(' ');
            s.push('(');
            for cmd in &table.items {
                s.push_str(cmd);
            }
            s.push(')');
        }

        return s;
    }

    /*****/

    /// A utility function that creates an actual schema handler
    fn make(s: Option<&str>) -> Schema {
        return Schema {
            schema: match s {
                Some(s) => Some(String::from(s)),
                _ => None,
            },
            jobs: Vec::new(),
        };
    }
}

/*
# withSchema
knex.schema.withSchema('public').createTable('users', function (table) {
  table.increments();
})
Outputs:
create table "public"."users" ("id" serial primary key)
    
# createTable

knex.schema.createTable('users', function (table) {
  table.increments();
  table.string('name');
  table.timestamps();
})
Outputs:
create table "users" ("id" serial primary key, "name" varchar(255), 
"created_at" timestamptz, "updated_at" timestamptz)


# createTableIfNotExists

knex.schema.createTableIfNotExists('users', function (table) {
  table.increments();
  table.string('name');
  table.timestamps();
})
Outputs:
create table if not exists "users" ("id" serial primary key, "name" varchar(255), 
"created_at" timestamptz, "updated_at" timestamptz)


# renameTable

knex.schema.renameTable('users', 'old_users')
Outputs:
alter table "users" rename to "old_users"


# dropTable

knex.schema.dropTable('users')
Outputs:
drop table "users"


# TODO: hasColumn
# TODO: hasTable


# dropTableIfExists

knex.schema.dropTableIfExists('users')
Outputs:
drop table if exists "users"


# table

knex.schema.table('users', function (table) {
  table.dropColumn('name');
  table.string('first_name');
  table.string('last_name');
})
Outputs:
alter table "users" add column "first_name" varchar(255), 
add column "last_name" varchar(255);
alter table "users" drop column "name"

# raw
*/
