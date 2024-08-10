-- Your SQL goes here
CREATE TABLE "projects"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"user_id" INTEGER NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

CREATE TABLE "users"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"age" INTEGER NOT NULL
);

