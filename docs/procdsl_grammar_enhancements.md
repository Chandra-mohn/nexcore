# ProcDSL Grammar Enhancements

19 grammar gaps fixed in `grammar/nexflow/ProcDSL.g4`.
Reference: `~/workspace/nexflow/nexflow-toolchain/docs/PROC-GRAMMAR-GAP-ANALYSIS.md`

---

## 1. Join -- Asymmetric Keys (GAP-01)

**Before:** Both sides must share the same field name.
```
join orders with customers on customer_id within 5 minutes
```

**After:** Left and right sides can have different field names.
```
join orders with customers
    on order.cust_id = customer.customer_id
    within 5 minutes

// Composite asymmetric keys
join transactions with accounts
    on txn.account_num = acct.id, txn.currency = acct.base_currency
    within 10 minutes
```

Symmetric form still works:
```
join orders with customers on customer_id within 5 minutes
```

---

## 2. INTO Clauses on Stream-Producing Operators (GAP-02/04/06/07/16/19)

**Before:** Operators that produce new streams have no way to name the output.
Code generator synthesizes names like `joined0Stream`, `windowed0Stream`.

**After:** All stream-producing operators support optional `INTO` for output naming.

### Join INTO (GAP-02)
```
join orders with customers on customer_id within 5 minutes type inner
    into enriched_orders

transform enriched_orders using calculate_totals into final_orders
```

### Window INTO (GAP-04)
```
window tumbling 1 hour
    key by card_holder_id
    aggregate
        count() as hourly_count
        sum(amount) as hourly_total
    end
    into velocity_metrics

transform velocity_metrics using score_velocity into scored_stream
```

### Aggregate INTO (GAP-06)
```
aggregate using daily_summary
    from batch_1, batch_2
    timeout 30 seconds
    into combined_summary
```

### SQL INTO (GAP-07)
```
sql ```
    SELECT account_id, SUM(amount) as total
    FROM transactions
    GROUP BY account_id
    HAVING SUM(amount) > 10000
``` as HighValueAccounts into high_value_stream
```

### Parallel INTO (GAP-16)
```
parallel enrichment_fan_out
    timeout 30 seconds
    require_all true

    branch geo_enrichment
        enrich using geo_service on postal_code
    end

    branch credit_enrichment
        enrich using credit_bureau on ssn
    end
end into fully_enriched_stream
```

### Deduplicate INTO (GAP-19)
```
deduplicate by order_id window 24 hours
    on_duplicate
        emit to duplicate_sink
    end
    into unique_orders
```

### Call INTO (GAP-19)
```
call ml_service "fraud_detection"
    features: { amount, merchant_category, time_of_day }
    timeout 2 seconds
    into scored_transactions
```

---

## 3. Window Composite Key (GAP-05)

**Before:** Single field only.
```
window tumbling 1 hour
    key by customer_id
```

**After:** Multiple fields for composite keys.
```
window tumbling 1 hour
    key by customer_id, region, product_type
    aggregate
        sum(amount) as total
    end
```

---

## 4. Join SELECT + Schema (GAP-08, GAP-18)

**Before:** Join output includes all fields from both sides. No projection.

**After:** Project specific fields and declare output schema.
```
join orders with customers on customer_id within 5 minutes type inner
    select order_id, amount, customer_name, tier
    schema enriched_order
    into enriched_orders
```

---

## 5. Standalone Filter Operator (GAP-09)

**Before:** Filter only available on `receive`. Mid-pipeline filtering requires
verbose `route when ... otherwise continue`.

**After:** Standalone `filter` for any point in the pipeline.
```
receive records from kafka "raw_events" into raw_stream

// Mid-pipeline filter
filter raw_stream when amount > 1000 into high_value_stream

// Filter without naming input (acts on current stream)
filter when status != "cancelled" into active_orders
```

---

## 6. BETWEEN Expression (GAP-10)

**Before:** Verbose compound comparison.
```
route when amount >= 100 and amount <= 500
    to mid_range_processing
```

**After:** Clean range syntax.
```
route when amount between 100 and 500
    to mid_range_processing
```

---

## 7. Pattern Matching (GAP-11)

**Before:** Only `contains()` available.
```
filter when contains(email, "@company.com") into internal_emails
```

**After:** Full pattern matching support.
```
// SQL-style LIKE
filter when email like "%@company.com" into company_emails

// Regex matching
filter when phone matches "^\\+1[0-9]{10}$" into us_phones

// Prefix/suffix checks
filter when starts_with(name, "Dr.") into doctors
filter when ends_with(filename, ".csv") into csv_files
```

---

## 8. CASE/WHEN Expression (GAP-14)

**Before:** Only ternary for two-way branching.
```
set tier = amount > 10000 ? "premium" : "standard"
```

**After:** Multi-branch conditional expression.
```
set tier = case
    when amount > 50000 then "platinum"
    when amount > 10000 then "gold"
    when amount > 1000  then "silver"
    else "bronze"
end

// With a scrutinee expression
set region_name = case region_code
    when "NA" then "North America"
    when "EU" then "Europe"
    when "AP" then "Asia Pacific"
    else "Other"
end
```

---

## 9. Null Handling Functions (GAP-15)

**Before:** No way to provide defaults for null values.

**After:** `coalesce()` and `ifnull()`.
```
set display_name = coalesce(preferred_name, legal_name, "Unknown")
set discount_rate = ifnull(customer_discount, 0.0)
set shipping_addr = coalesce(override_addr, default_addr, billing_addr)
```

---

## 10. JDBC Connector (GAP-20)

**Before:** Database access only via generic IDENTIFIER catch-all.

**After:** Dedicated `jdbc` connector type.
```
receive accounts from source into account_stream
    from jdbc "jdbc:postgresql://db:5432/banking"
    schema account_record

emit results to output
    to jdbc "jdbc:postgresql://db:5432/analytics"
```

---

## 11. Type Casting Functions (GAP-21)

**Before:** No type conversion in expressions.

**After:** Type casting for common conversions.
```
set amount_cents = to_int(amount * 100)
set display_amount = to_string(amount)
set rate = to_decimal(rate_string)
set txn_date = to_date(date_string)

// Generic cast
set priority = cast(priority_string, "integer")
```

---

## 12. GROUP BY Without Window -- Batch Mode (GAP-22)

**Before:** Aggregation only inside window blocks (stream-oriented).

**After:** Standalone GROUP BY for batch-mode processing.
```
process daily_report
    mode batch

    receive transactions from source into txn_stream
        from csv "/data/daily/transactions.csv"

    group by department, region
        aggregate
            sum(amount) as total_amount
            count() as txn_count
            avg(amount) as avg_amount
        end
        having total_amount > 10000
        into department_summaries

    emit department_summaries to output
        to csv "/data/reports/dept_summary.csv"
end
```

---

## 13. ORDER BY / LIMIT -- Batch Mode (GAP-23)

**Before:** No sorting or result limiting.

**After:** ORDER BY with ASC/DESC and optional LIMIT.
```
process top_customers
    mode batch

    receive customers from source into customer_stream
        from jdbc "jdbc:postgresql://db/crm"

    group by region
        aggregate
            sum(revenue) as total_revenue
        end
        into region_totals

    order by total_revenue desc
        limit 100
        into top_regions

    emit top_regions to output
        to parquet "/data/lake/top_regions/"
end
```

Multi-field sorting:
```
order by department asc, hire_date desc, employee_id asc
    limit 500
    into sorted_employees
```

---

## Summary

| # | Enhancement | Category |
|---|------------|----------|
| 1 | Asymmetric join keys | Rule modification |
| 2 | INTO on 7 operators | Consistency |
| 3 | Composite window key | Rule modification |
| 4 | Join SELECT + schema | New clauses |
| 5 | Standalone filter | New statement |
| 6 | BETWEEN expression | Expression enhancement |
| 7 | LIKE/MATCHES/STARTS_WITH/ENDS_WITH | Expression enhancement |
| 8 | CASE/WHEN expression | Expression enhancement |
| 9 | COALESCE/IFNULL | Function addition |
| 10 | JDBC connector | Connector addition |
| 11 | Type casting functions | Function addition |
| 12 | GROUP BY (batch) | New statement |
| 13 | ORDER BY/LIMIT (batch) | New statement |
