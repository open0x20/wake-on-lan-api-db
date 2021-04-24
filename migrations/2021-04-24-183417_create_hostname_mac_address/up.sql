CREATE TABLE hostname_mac_address
(
    hostname VARCHAR(255) NOT NULL,
    mac_address CHAR(17) NOT NULL,

    PRIMARY KEY(hostname, mac_address)
);