-- This is a table that is able to be used by the dashboard or publicly facing programs that might want to "simulate"
-- the neural network
CREATE VIEW PublicNode AS
    SELECT
        ModelId, Layer, X, Y, Weight, Bias
FROM Node
WHERE TargetNetwork = 0