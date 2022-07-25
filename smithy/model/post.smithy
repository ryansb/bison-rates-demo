namespace gov.ny.buffalo

@http(uri: "/prod/bison", method: "POST")
operation CreateBison {
    input: CreateBisonInput,
    output: CreateBisonOutput
}

@input
structure CreateBisonInput {
    @required
    herd: Name,

    @required
    name: Name,

    rank: HerdRank,

    tags: BisonTags
}

@output
structure CreateBisonOutput {
    @required
    id: BisonId,

    @required
    name: Name,

    @required
    herd: Name,

    rank: HerdRank,

    tags: BisonTags,

    @httpHeader("X-Bison-Version")
    version: ServiceVersion
}
