### Find the station minigame for orientation. 
Many improvements from last year's O-Quest in order to prevent cheating :)

Stations display pre-generated QR Codes mapped to a rough timespan, flashing a new one every ~10 seconds. When codes are scanned by the mobile app, server side verifies authenticity and disables that code from being redeemed. Location verification also doen through app, minimizing power consumption on stations.


## [For now] Station Parts List:
- RP2040 microcontroller - https://www.canakit.com/raspberry-pi-rp2040.html?cid=usd&src=raspberrypi ($1)
- 1.22 Inch E-Ink display @ https://www.good-display.com/product/548.html
- 5000mAh Li-Polymer Batteries 3.7V @ https://www.alibaba.com/pla/Professional-Rechargeable-Lithium-Batteries-5000mAh-Li-polymer_1601039428492.html?mark=google_shopping&biz=pla&searchText=pouch+lithium+ion+batteries&product_id=1601039428492&pcy=us_en
- Container of sorts (laser cut MDF, acrylic, etc., TBD)
- 


## DB Schema in Discussion - PostgreSQL
# user-side
users (
    id bigint generated always as identity primary key,
    display_name text,
    avatar_url text,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
)

user_dorm_memberships (
    id bigint generated always as identity primary key,
    user_id bigint not null references users(id),
    dorm_id bigint not null references dorms(id),
    starts_at timestamptz not null,
    ends_at timestamptz,
    check (ends_at is null or ends_at > starts_at)
)

fy_dorms(
    id smallint generated always as identity primary key,
    name text not null unique
)

user_identities (
    id bigint generated always as identity primary key,
    user_id bigint not null references users(id),
    provider text not null,              -- e.g. 'cmu_shibboleth'
    external_subject text not null,      -- ePPN, trusted value from provider
    unique (provider, external_subject),
    unique (user_id, provider)
)

# game/station side