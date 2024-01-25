# master

- Remove support for "skip_serializing", "skip_serializing_if" and "skip_deserializing".
    - Initially supporting these by skipping a field was a mistake. If a user wishes to skip a field, they can still
      annotate it with `#[ts(skip)]`