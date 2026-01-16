from typing import Annotated

from mcp.server.fastmcp import FastMCP
from mcp.server.fastmcp.prompts.base import Message, UserMessage
from pydantic import Field

mcp = FastMCP("DocumentMCP", log_level="ERROR")

docs = {
    "deposition.md": "This deposition covers the testimony of Angela Smith, P.E.",
    "report.pdf": "The report details the state of a 20m condenser tower.",
    "financials.docx": "These financials outline the project's budget and expenditures.",
    "outlook.pdf": "This document presents the projected future performance of the system.",
    "plan.md": "The plan outlines the steps for the project's implementation.",
    "spec.txt": "These specifications define the technical requirements for the equipment.",
}


@mcp.tool(name="read_document", description="Reads a document from the server.")
def read_doc(doc_id: Annotated[str, Field(description="The id of the doc to read.")]):
    if doc_id not in docs:
        raise ValueError(f"Doc {doc_id} not found.")

    return docs[doc_id]


@mcp.tool(name="edit_document", description="Edits a document on the server.")
def edit_doc(
    doc_id: Annotated[str, Field(description="The id of the doc to edit.")],
    old_str: Annotated[str, Field(description="The string to replace.")],
    new_str: Annotated[str, Field(description="The replacement string.")],
):
    if doc_id not in docs:
        raise ValueError(f"Doc {doc_id} not found.")
    docs[doc_id] = docs[doc_id].replace(old_str, new_str)


@mcp.resource("docs://documents", mime_type="application/json")
def list_docs() -> list[str]:
    return list(docs.keys())


@mcp.resource("docs://documents/{doc_id}", mime_type="text/plain")
def get_doc(doc_id: str) -> str:
    if doc_id not in docs:
        raise ValueError(f"Doc {doc_id} not found.")

    return docs[doc_id]


@mcp.prompt(
    name="format",
    description="Rewrites the contents of the document in markdown format.",
)
def format_doc(
    doc_id: Annotated[str, Field(description="The id of the document to format.")],
) -> list[Message]:
    prompt = f"""
    Your goal is to reformat a document to be written with markdown syntax.
    
    The id of the document you need to reformat is:
    <document_id>
    {doc_id}
    </document_id>

    Add in headers, bullet points, tables, etc as necessary.
    Use the 'edit_document' tool to edit the document, then return the reformatted content.
    """

    return [UserMessage(prompt)]


if __name__ == "__main__":
    mcp.run(transport="stdio")
