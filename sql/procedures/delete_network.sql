create procedure delete_network(@NetworkId int)
AS
begin
    DELETE FROM Bias WHERE NetworkId = @NetworkId;
    DELETE FROM Weight WHERE NetworkId = @NetworkId;
    DELETE FROM Network WHERE NetworkId = @NetworkId;
end
