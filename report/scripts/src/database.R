dotenv::load_dot_env()
library(RJDBC)

connectToDatabase <- function() {
  driver <- JDBC(
    "com.microsoft.sqlserver.jdbc.SQLServerDriver",
    Sys.getenv("JDBC_PATH")
  )

  conn <- dbConnect(
    drv = driver,
    sprintf(
      "jdbc:sqlserver://%s:%s;databaseName=%s;encrypt=false",
      Sys.getenv("SQL_HOST"),
      Sys.getenv("SQL_PORT"),
      Sys.getenv("SQL_DATABASE")
    ),
    Sys.getenv("SQL_USERNAME"),
    Sys.getenv("SQL_PASSWORD")
  )

  return(conn)
}

get_epochs <- function(...) {
  args <- list(...)
  conn <- connectToDatabase()

  dataframes <- lapply(args, function(modelId) {
    rs <- dbSendQuery(conn, "SELECT * FROM Epoch WHERE ModelId = ?", modelId)
    df <- dbFetch(rs, n = -1)
    return(df)
  })
  dbDisconnect(conn)

  joined_df <- do.call(rbind, dataframes)

  return(joined_df)
}

get_evaluation_data <- function(...) {
  args <- list(...)
  conn <- connectToDatabase()

  dataframes <- lapply(args, function(modelId) {
    rs <- dbSendQuery(conn,"SELECT * FROM EvaluationData WHERE ModelId = ?", modelId)
    df <- dbFetch(rs, n = -1)
    return(df)
  })
  dbDisconnect(conn)

  joined_df <- do.call(rbind, dataframes)

  return(joined_df)
}