from strictdoc.core.project_config import ProjectConfig, SourceNodesEntry

def create_config() -> ProjectConfig:
    config = ProjectConfig(
        project_features=[
            "REQUIREMENT_TO_SOURCE_TRACEABILITY",
            "SOURCE_FILE_LANGUAGE_PARSERS",
            "PROJECT_STATISTICS_SCREEN",
        ],
        source_nodes=[
            SourceNodesEntry(
                path="",
                uid="SRC-NODES-BASE",
                node_type="REQUIREMENT",
            )
        ],
    )
    return config
