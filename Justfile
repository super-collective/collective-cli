default: schema

schema: schema-potoc schema-fellowship schema-generic

schema-potoc:
	cargo r -q -- schema evidence --collective potoc --output schema/
	cargo r -q --frozen -- schema join-request --collective potoc --output schema/

schema-fellowship:
	cargo r -q --frozen -- schema evidence --collective fellowship --output schema/
	cargo r -q --frozen -- schema join-request --collective fellowship --output schema/

schema-generic:
	cargo r -q --frozen -- schema generic-evidence --output schema/
	cargo r -q --frozen -- schema generic-join-request --output schema/
