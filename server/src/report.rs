use eyre::{Context, ContextCompat, Result, ensure, eyre};
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

#[derive(Debug, Clone)]
pub struct LineRegion {
    pub file_path: String,
    pub start_line: i32,
    pub end_line: i32,
    pub start_column: i32,
    pub end_column: i32,
    pub statements: i32,
    pub executed: i32,
}

/// Reads the report and parses it. The replies are sent to the reply sender.
/// If the report is invalid, an error is returned.
pub async fn parse_go_report(mut read: impl AsyncBufRead + Unpin) -> Result<Vec<LineRegion>> {
    let mut vec = Vec::with_capacity(1024);
    let mut buf = String::new();
    read.read_line(&mut buf)
        .await
        .wrap_err("could not read a line")?;
    buf.clear(); // skip the mode line
    while read
        .read_line(&mut buf)
        .await
        .wrap_err("failed to read line")?
        != 0
    {
        vec.push(
            parse_go_line(&buf)
                .wrap_err_with(|| format!("failed to parse line: {}", buf.trim()))?,
        );
        buf.clear();
    }

    Ok(vec)
}

fn parse_go_line(line: &str) -> Result<LineRegion> {
    // filepath:start_line.start_column,end_line.end_column statements executed
    // example: pkg/file.go:1.2,3.4 5 6
    let mut parts = line.trim().split(' ');
    let first = parts
        .next()
        .ok_or_else(|| eyre!("missing first component"))?;
    let statements = parts.next().ok_or_else(|| eyre!("missing statements"))?;
    let statements = statements
        .parse()
        .wrap_err_with(|| format!("failed to parse statements: {}", statements))?;
    ensure!(statements >= 0, "cannot have negative statement count");
    let executed = parts.next().ok_or_else(|| eyre!("missing executed"))?;
    let executed = executed
        .parse()
        .wrap_err_with(|| format!("failed to parse executed: {}", executed))?;
    ensure!(executed >= 0, "cannot have negative executed count");

    let mut parts = first.split(':');
    // parts: ["pkg/file.go", "1.2,3.4"]
    let file_path = parts
        .next()
        .ok_or_else(|| eyre!("missing file path"))?
        .to_string();

    let mut parts = parts
        .next()
        .ok_or_else(|| eyre!("missing line parts"))?
        .split(',');
    let mut start_parts = parts
        .next()
        .ok_or_else(|| eyre!("missing start parts"))?
        .split('.');
    // start_parts: [1, 2]
    let start_line = start_parts.next().wrap_err("missing start line")?;
    let start_line = start_line
        .parse()
        .wrap_err_with(|| format!("failed to parse start line: {}", start_line))?;
    ensure!(start_line >= 1, "start line must be >= 1");
    let start_column = start_parts
        .next()
        .ok_or_else(|| eyre!("missing start column"))?;
    let start_column = start_column
        .parse()
        .wrap_err_with(|| format!("failed to parse start column: {}", start_column))?;
    ensure!(start_column >= 0, "start column must be >= 0");

    let mut end_parts = parts
        .next()
        .ok_or_else(|| eyre!("missing end parts"))?
        .split('.');
    // end_parts: [3, 4]
    let end_line = end_parts.next().wrap_err("missing end line")?;
    let end_line = end_line
        .parse()
        .wrap_err_with(|| format!("failed to parse end line: {}", end_line))?;
    ensure!(
        end_line >= start_line,
        "end line must be >= start_line ({start_line})"
    );
    let end_column = end_parts
        .next()
        .ok_or_else(|| eyre!("missing end column"))?;
    let end_column = end_column
        .parse()
        .wrap_err_with(|| format!("failed to parse end column: {}", end_column))?;
    ensure!(end_column >= 0, "end column must be >= 0");

    Ok(LineRegion {
        file_path,
        start_line,
        start_column,
        end_line,
        end_column,
        statements,
        executed,
    })
}
