

DROP TABLE IF EXISTS "customers";
DROP SEQUENCE IF EXISTS customers_id_seq;
CREATE SEQUENCE customers_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."customers" (
    "id" integer DEFAULT nextval('customers_id_seq') NOT NULL,
    "company_name" text NOT NULL,
    "vat_id" text NOT NULL,
    "address" text NOT NULL,
    "area" text NOT NULL,
    "legal_name" text NOT NULL,
    "emails" jsonb NOT NULL,
    "phones" jsonb,
    "postcode" integer,
    "website" text
) WITH (oids = false);

INSERT INTO "customers" ("id", "company_name", "vat_id", "address", "area", "legal_name", "emails", "phones", "postcode", "website") VALUES
(10,	'moon',	'moon78282',	'in to the moon',	'in to the moon',	'moon_company',	'{"sales_email": "", "primary_email": "", "accounting_email": "", "cucstomer_suport": ""}',	'[]',	2222,	'None'),
(3,	'update_name',	'moon7565464',	'in to the moon',	'stars',	'moon_company',	'{"sales_email": "", "primary_email": "", "accounting_email": "", "cucstomer_suport": ""}',	'[]',	2222,	'None');


