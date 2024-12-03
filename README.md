# Streaming platform
This is a one month project to build streaming platform using rust as the backend.

## Crates used
- axum
- dotenvy


## idea/ description of the project.

We would create a streaming platform where by users can log in and stream with one another. Apart from streaming they should be able to chat with one another.

For the backend we would have the following routes: 

#### 1. Authentication and User Management
**User Registration:**
- POST `/api/register`

**User Login:**
- POST `/api/login`

**User Logout:**
- POST `/api/logout`

**Profile Management:**
- GET `/api/user/{user_id}`
    - Fetches user details.
- PUT `/api/user/{user_id}`
    - Updates profile information.


#### 2. Stream Management
**Create a Stream:**
- POST `/api/streams`
    - Initializes a new stream with details like `title`, `category`, and `tags`.

**Start a Stream:**
- POST `/api/streams/{stream_id}/start`
    - Begins broadcasting the stream.

**Stop a Stream:**
- POST `/api/streams/{stream_id}/stop`
    - Ends the stream.

**Fetch Stream Details:**
- GET `/api/streams/{stream_id}`
    - Retrieves details about a specific stream.

**List Active Streams:**
- GET `/api/streams`
