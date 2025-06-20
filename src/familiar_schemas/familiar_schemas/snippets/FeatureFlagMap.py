# generated by datamodel-codegen:
#   filename:  tmpl4c9strw.json
#   timestamp: 2025-06-21T16:54:29+00:00

from __future__ import annotations

from typing import Mapping

from pydantic import ConfigDict, RootModel


class FeatureFlagMap(RootModel[Mapping[str, bool] | None]):
    """
    Schema for FeatureFlagMap
    """

    model_config = ConfigDict(
        frozen=True,
    )
    root: Mapping[str, bool] | None
