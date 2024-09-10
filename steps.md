When starting a software project you might have some large grand idea. This often
can become can cause hesitation with respect to getting out the gate. It is crucial
to take the big vision and start off by distilling this vision into it's most basic components
This will provide a spring board that can help shape the software development process
and allows for continuous iterate development and deployment.

For this project we are building a subscription based newsletter service

Once a general idea has been laid out it is very helpful to leverage 'user stories' with a
"as a ->"
"i want to ->"
"so that ->"
approach

As a user, I want to subscribe to the newsletter, so that i can receive email
updates

As a author, I want to send email notifications when new content is available,
so that my followers can stay up to date

This spells out the baby form of the service we are aiming to provide and will
guide the beginning of the design process

We need to create some type of web base form, that a user can submit their information,
it will be stored persistently in a database, then this database will be used to
grab user information and send emails automatically when it is time to notify the users
of new content

We will be using rust as our backend:
- actix-web : as web framework
- tokio : as our async framework 
- Postgres : as our Database
- sqlx : rust-postgres interface


* Start Postgres Database -> ./scripts/init_db.sh
   -
