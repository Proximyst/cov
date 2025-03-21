// This file is auto-generated by @hey-api/openapi-ts

/**
 * The input report was invalid. There are no details as this should be regarded as an unrecoverable error; a new upload would be required.
 */
export type InvalidReport = 'LineNumberInvalid' | 'StatementsInvalid' | 'ParseError';

/**
 * The server is alive. It contains when the request was processed, such that latency can be measured.
 */
export type Pong = {
    /**
     * The time at which the request was processed by our handlers. This allows the client to measure latency between the client and server after all middlewares and similar are complete.
     */
    'processed-timestamp': string;
};

/**
 * A single code region in a report.
 *
 * This can be a partial line, a full line, partial multiple lines, or full multiple lines.
 */
export type Region = {
    /**
     * The number of executions this region had.
     *
     * This is not limited by the number of statements in the region.
     */
    executions: number;
    /**
     * The file path this region belongs to. This can be either a relative path (usually from the repository root) or an absolute path.
     *
     * This is an [`Arc`] to reduce memory usage: one file can have thousands of regions, and as such share the same memory region.
     */
    file: string;
    /**
     * Where the region starts. The tuple is `(line, column)`.
     */
    from: [
        unknown,
        unknown
    ];
    /**
     * The number of statements in this region. The idea of a statement can vary between languages, e.g. individual function calls or JVM byte code instructions.
     *
     * This is not limited by the number of executions.
     */
    statements: number;
    /**
     * Where the region ends. The tuple is `(line, column)`.
     *
     * If the column is `0`, the region ended on the previous line's end. This is an optimisation to not have to read the actual file to determine the line's length. As such, the line can be larger than the file is, by 1.
     *
     * The line is always greater than or equal to the `from` line.
     */
    to: [
        unknown,
        unknown
    ];
};

/**
 * An aggregate report of covered files.
 */
export type Report = {
    /**
     * Every code region in this report.
     */
    regions: Array<Region>;
};

export type ResultOfReportOrInvalidReport = {
    Ok: Report;
} | {
    Err: InvalidReport;
};

export type PostTestData = {
    body: unknown;
    path?: never;
    query?: never;
    url: '/test';
};

export type PostTestResponses = {
    200: ResultOfReportOrInvalidReport;
};

export type PostTestResponse = PostTestResponses[keyof PostTestResponses];

export type GetV0PingData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/v0/ping';
};

export type GetV0PingResponses = {
    /**
     * A successful pong.
     */
    200: Pong;
};

export type GetV0PingResponse = GetV0PingResponses[keyof GetV0PingResponses];

export type ClientOptions = {
    baseUrl: 'http://localhost:8080' | (string & {});
};