# Question

Design an **Online Food Delivery Platform** (like Swiggy/Zomato) using an **event-driven architecture**.

## Functional Requirements

- Users can browse restaurants and menus.
- Users place orders.
- Restaurants accept or reject orders.
- Delivery partners are assigned to accepted orders.
- Payments are processed for each order.
- Users can track their order in real time (status, ETA, driver location).
- Notifications (email/SMS/push) are sent at each major step.
- Analytics/loyalty/fraud-detection services need to observe every order event.

## Clients

- Customer mobile app
- Restaurant dashboard (web)
- Delivery partner app

## Design Goals

Walk through the design and explain, with justification, how you would apply each of the following concepts to this system:

1. **BFF (Backend for Frontend)** — how do you serve three very different clients without one bloated general-purpose API?
2. **Message Queue** — how do you keep the user-facing request fast while slow background work (emails, SMS, invoices) happens off the critical path?
3. **Stream** — how do multiple independent services (analytics, loyalty, fraud detection, notifications) all react to the same sequence of order events without tightly coupling to the order service?
4. **CQRS** — order placement is write-heavy and order tracking is read-heavy with very different access patterns; how do you avoid one data model serving both badly?
5. **Saga** — placing an order spans reserving food, charging payment, and assigning a delivery partner across separate services with no shared database transaction; how do you keep this consistent, and what happens when one step fails partway through?

Then describe how these five pieces compose into a single end-to-end architecture, from the moment a customer taps "Place Order" to the moment the order is delivered.
