namespace gov.ny.buffalo

// the /{stage}/ is due to an artifact in httpLabel deserialization
// and is not used
@http(uri: "/{stage}/bison/{herd}", method: "GET")
@readonly
operation ListBison {
    input: ListBisonInput,
    output: ListBisonOutput
}

@input
structure ListBisonInput {
    @required
    @httpLabel
    herd: Name,

    @required
    @httpLabel
    stage: String,
}

structure BisonItem {
    @required
    id: BisonId,

    @required
    name: String,

    rank: HerdRank,

    tags: BisonTags,
}

list HerdMembers {
    member: BisonItem
}

@output
structure ListBisonOutput {
    members: HerdMembers,

    @required
    max: HerdRank,

    @httpHeader("X-Bison-Version")
    version: ServiceVersion,
}
