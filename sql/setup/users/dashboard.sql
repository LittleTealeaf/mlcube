CREATE LOGIN Dashboard WITH PASSWORD = 'dashboard-login'

CREATE USER Dashboard FOR LOGIN Dashboard;
GRANT CONNECT TO Dashboard;

GRANT SELECT ON Models TO Dashboard;
GRANT SELECT ON EvaluationMoves TO Dashboard;
GRANT SELECT ON Evaluations TO Dashboard;
GRANT SELECT ON Epochs TO Dashboard;
