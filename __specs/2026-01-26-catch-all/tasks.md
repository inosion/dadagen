- [ ] I notice the logic for determining field types (inspection) may be duplicated in core and UIU, consolidate 
feature:  { "uniform" | "normal" | "exponential" } randomise modifiers 
feature: weighted list_weighted_constraint = { "weighted" ~ WSNL* ~ "=" ~ WSNL* ~ ("true" | "false") }
feature: seed number_seed_constraint = { "seed" ~ WSNL* ~ "=" ~ WSNL* ~ number }
