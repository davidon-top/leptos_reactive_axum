# whoami
Library providing utilities for using leptos_reactive runtime inside axum handlers. Also automaticly uses provide_context to make the request Parts available inside any function called by the handler.

# But why?
When using something like the [rstml-component](https://crates.io/crates/rstml-component) crate it is usefull to be able to extract anything from parts inside any component. For example inside nav bar extract the path to be able to highlight active route. Additionally you can create a middleware layer that can add to the context things like IsLoggedIn or other information about the request that is used within more then one function/component called by the handler.
