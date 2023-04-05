CREATE LOGIN Agent WITH PASSWORD = 'mlcube20123';

CREATE USER Agent FOR LOGIN Agent;
GRANT CONNECT TO Agent;

GRANT INSERT ON Models TO Agent;
GRANT SELECT ON Models TO Agent;

GRANT DELETE ON Nodes TO Agent;
GRANT SELECT ON Nodes TO Agent;
GRANT INSERT ON Nodes TO Agent;

GRANT INSERT ON Evaluations TO Agent;
GRANT INSERT ON EvaluationMoves TO Agent;

GRANT INSERT ON Epochs TO Agent;



GRANT EXECUTE ON get_current_epoch TO Agent;
