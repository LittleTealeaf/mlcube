CREATE PROCEDURE get_nodes(@ModelId INT) AS
BEGIN
    SELECT Layer, NodeIndex, Weight, Bias FROM Nodes WHERE ModelId = @ModelId
end