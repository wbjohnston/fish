-- Add up migration script here

create function game_notify_on_player_change()
returns trigger
language plpgsql
as $$
	declare
		row RECORD;
	BEGIN
		CASE TG_OP
		WHEN 'INSERT', 'UPDATE' THEN
			row := NEW;
		WHEN 'DELETE' THEN
			row := OLD;
		ELSE
			RAISE EXCEPTION 'Unknown TG_OP: "%". Should not occur!', TG_OP;
		END CASE;


		PERFORM pg_notify('game_notifications', format('{"gameId": "%s"}', row.game_id));
		return null;
	END;
$$;

create constraint trigger player_notify
	after
		insert
		or update
		or delete
	on players not DEFERRABLE
	for each row
		execute procedure game_notify_on_player_change();


-- Add up migration script here
