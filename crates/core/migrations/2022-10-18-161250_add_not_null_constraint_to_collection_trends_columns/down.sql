ALTER TABLE collection_trends ALTER COLUMN _1d_volume DROP NOT NULL,
ALTER COLUMN _7d_volume DROP NOT NULL,
ALTER COLUMN _30d_volume DROP NOT NULL,
ALTER COLUMN _prev_1d_volume DROP NOT NULL,
ALTER COLUMN _prev_7d_volume DROP NOT NULL,
ALTER COLUMN _prev_30d_volume DROP NOT NULL,
ALTER COLUMN _1d_sales_count DROP NOT NULL,
ALTER COLUMN prev_1d_sales_count DROP NOT NULL,
ALTER COLUMN _7d_sales_count DROP NOT NULL,
ALTER COLUMN prev_7d_sales_count DROP NOT NULL,
ALTER COLUMN _30d_sales_count DROP NOT NULL,
ALTER COLUMN prev_30d_sales_count DROP NOT NULL,
ALTER COLUMN floor_price DROP NOT NULL,
ALTER COLUMN prev_1d_floor_price DROP NOT NULL,
ALTER COLUMN prev_7d_floor_price DROP NOT NULL,
ALTER COLUMN prev_30d_floor_price DROP NOT NULL,
ALTER COLUMN _1d_volume_change DROP NOT NULL,
ALTER COLUMN _7d_volume_change DROP NOT NULL,
ALTER COLUMN _30d_volume_change DROP NOT NULL,
ALTER COLUMN _1d_floor_price_change DROP NOT NULL,
ALTER COLUMN _7d_floor_price_change DROP NOT NULL,
ALTER COLUMN _30d_floor_price_change DROP NOT NULL,
ALTER COLUMN _1d_sales_count_change DROP NOT NULL,
ALTER COLUMN _7d_sales_count_change DROP NOT NULL,
ALTER COLUMN _30d_sales_count_change DROP NOT NULL,
ALTER COLUMN _1d_marketcap_change DROP NOT NULL,
ALTER COLUMN _7d_marketcap_change DROP NOT NULL,
ALTER COLUMN _30d_marketcap_change DROP NOT NULL;