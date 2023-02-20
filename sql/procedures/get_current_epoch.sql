create procedure get_current_epoch(@ModelId int) AS
begin
    SELECT *
    FROM Epochs E
    WHERE E.ModelId = @ModelId
      AND Epoch = (SELECT MAX(Epoch)
                   FROM Epochs
                   WHERE Epochs.ModelId = @ModelId)
end

-- SELECT * FROM Epochs WHERE ModelId = 1 AND Epoch = (SELECT MAX(Epoch) FROM Epochs WHERE ModelId = 1)