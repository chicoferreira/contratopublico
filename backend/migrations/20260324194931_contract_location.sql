ALTER TABLE contracts
ADD COLUMN execution_place_array TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[];

UPDATE contracts
SET execution_place_array = ARRAY(
    SELECT split.value
    FROM (
        SELECT
            BTRIM(part) AS value,
            MIN(ord) AS first_seen
        FROM UNNEST(STRING_TO_ARRAY(execution_place, '<BR/>')) WITH ORDINALITY AS parts(part, ord)
        WHERE BTRIM(part) <> ''
        GROUP BY BTRIM(part)
    ) AS split
    ORDER BY split.first_seen
);

ALTER TABLE contracts
ALTER COLUMN execution_place_array DROP DEFAULT;

ALTER TABLE contracts
DROP COLUMN execution_place;

ALTER TABLE contracts
RENAME COLUMN execution_place_array TO execution_places;
