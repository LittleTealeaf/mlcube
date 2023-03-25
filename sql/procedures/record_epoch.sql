CREATE PROCEDURE record_epoch(@ModelId INT,
    @Loss FLOAT,
    @Reward FLOAT)
AS
BEGIN
    DECLARE @Epoch AS INT;
    EXEC @Epoch = get_current_epoch @ModelId = @ModelId;

    INSERT INTO Epochs
        (ModelId, Epoch, Loss, Reward)
    VALUES
        (@ModelId, @Epoch, @Loss, @Reward)
END
