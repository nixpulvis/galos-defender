SELECT json_agg(t) FROM (

SELECT DISTINCT system_factions.*
FROM systems
JOIN system_factions ON systems.address = system_factions.system_address
JOIN factions ON system_factions.faction_id = factions.id
WHERE position &&& (SELECT
    ST_3DMakeBox(
        ST_MakePoint(ST_X(position)+20, ST_Y(position)+20, ST_Z(position)+20),
        ST_MakePoint(ST_X(position)-20, ST_Y(position)-20, ST_Z(position)-20))
    FROM systems
    WHERE name = 'MELIAE')

) t
