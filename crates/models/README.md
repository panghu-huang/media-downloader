# Models

models here choose to use `sqlx` directly to operate the database, rather than choose `diesel` this ORM framework, mainly because the learning cost of `diesel` is high, and the concept is complicated, and many Derive are needed, etc.

If you choose ORM, it is more inclined to use `prisma rust`, but prisma is easy to appear in integration tests [Server has closed the connection.](https://www.prisma.io/docs/orm/reference/error-reference#p1017) This problem, so in the end, `sqlx` was chosen.