○​ Setup instructions

    -- setup Rust
    -- Go to the dns_server folder run cargo run
    -- Server is at 127.0.0.1:3000, so make sure there are no other applications there
    -- YOu cna use Postman and curl commands to test



○​ Clear documentation of endpoints with examples



○​ Reasoning behind implementation decisions

-- Split code into different crates to make it as modular as possible, to make it easier to organize and test

-- Decided on a simple approach of locking the entire shared mutable state on a call to the in memory store (for simplicty, eventhoguh Actor Model might have been better)

-- Used a RwLock for DNS are more read heavy than write heavy

-- Next steps
    Write Ahead Log
    Persistennce between crashes



○​ Explicit indication/documentation if using AI tools (upload chat history or
documentation)

-- Used AI to debug some issues and as auto complete, did not rely on agents to code for me (though it might have been faster if I did)