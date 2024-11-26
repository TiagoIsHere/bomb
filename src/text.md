```js
// Connect to the backend API
connect backend "https://api.example.com";

// Fetch data from multiple endpoints
data users = fetch "/users" from backend;
data products = fetch "/products" from backend;

// Filter active users from the users dataset
filter activeUsers from users where active;

// Log the number of active users
print "Number of active users: " + size(activeUsers);

// Filter products based on stock availability
filter inStockProducts from products where stock > 0;

// Print the names of all in-stock products
print "Products in stock:";
for product in inStockProducts {
    log product.name;
}

// Conditional logic to handle data
if size(inStockProducts) == 0 {
    print "No products are currently in stock.";
} else if size(inStockProducts) < 5 {
    print "Low stock warning: Only a few products are available!";
} else {
    print "Stock levels are healthy.";
}

// Fetch orders data and process it
data orders = fetch "/orders";
filter highValueOrders from orders where total > 100;

// Save high-value orders to a local file
save highValueOrders to "high_value_orders.json";

// Loop through high-value orders and print details
print "High-value orders:";
for order in highValueOrders {
    print "Order ID: " + order.id + ", Total: $" + order.total;
}

// Example of transforming data with map
map orderSummaries from orders with order => {
    id: order.id,
    items: order.items.length,
    total: order.total
};

// Save order summaries to another file
save orderSummaries to "order_summaries.json";

// End of program
print "All operations completed successfully.";
```

```js
data apiToken = "asdf";

function get(url) {
    let res = fetch(url, {
        token: apiToken
    });
}

data x = do get "/test";
```

```js
data x = do {
    let res = fetch(url, {
        token: apiToken
    });
    return res;
};

filter y from x where active;
```
```js
function transaction_x() {
    let res = fetch(url, {
        token: apiToken
    });
    return res;
}

const x = transaction_x();

