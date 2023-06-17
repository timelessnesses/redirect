import fastapi
import starlette.middleware

import routes.redirect as redirect

app = fastapi.FastAPI(title="timelessnesses.api.redirect", docs_url="/")

app.include_router(
    redirect.ext,
)
