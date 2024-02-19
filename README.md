# Email Tracker
Simple server written in Rust that can be used to track how many times an email has been read.

## How it works
1. Making a `GET` request to any route (e.g. `localhost:3030/email-1`, `localhost:3030:/email-2`) return a transparent 1x1 image
2. Additionally, every time a `GET` request is serviced, it is logged to count the total number of views
3. By embedding a url (e.g. `localhost:3030/email-1`) as an image in an email, the image is loaded every time the email is read and is thus logged

Note that email tracking blockers exist and hinder the efficacy of using this approach to track emails.
