CREATE LOGIN Dashboard WITH PASSWORD = 'dashboard-login'

CREATE USER Dashboard FOR LOGIN Dashboard;
GRANT CONNECT TO Dashboard;

GRANT SELECT ON Model TO Dashboard;
GRANT SELECT ON EvaluationMove TO Dashboard;
GRANT SELECT ON Evaluation TO Dashboard;
GRANT SELECT ON Epoch TO Dashboard;
GRANT SELECT ON ModelInfo TO Dashboard;
GRANT SELECT ON GroupedEpoch TO Dashboard;
GRANT SELECT ON EvaluationInfo TO Dashboard;
GRANT SELECT on EvaluationRewardChange TO Dashboard;