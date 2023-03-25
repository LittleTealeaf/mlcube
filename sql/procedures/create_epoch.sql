CREATE PROCEDURE create_epoch(@ModelId INT,
    @Epoch INT,
    @Loss FLOAT,
    @Reward FLOAT)
AS

BEGIN
    INSERT INTO Epochs
        (ModelId, Epoch, Loss, Reward)
    OUTPUT
    inserted.EpochId
    VALUES
        (@ModelId, @Epoch, @Loss, @Reward)
END
