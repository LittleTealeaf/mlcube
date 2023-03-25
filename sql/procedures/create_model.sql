CREATE PROCEDURE create_model(@ModelName varchar(100),
    @GitHash varchar(40))
AS
BEGIN

    INSERT INTO Models
        (ModelName, GitHash)
    OUTPUT
    inserted.ModelId
    VALUES
        (@ModelName, @GitHash)


END
