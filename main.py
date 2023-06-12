import fastapi
import routes.redirect as redirect
import starlette.middleware
app = fastapi.FastAPI(title="timelessnesses.api.redirect",docs_url="/")

app.include_router(
    redirect.ext,
)