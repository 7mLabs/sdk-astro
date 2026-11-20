"""Check the file inventory written into a final 7mlabs Python wheel."""
import base64
import csv
import hashlib
import io


def verify_wheel_record(archive, record_name):
    files = {}
    for member in archive.infolist():
        if member.is_dir():
            continue
        if member.filename in files:
            raise ValueError(f"Duplicate file in wheel: {member.filename}")
        files[member.filename] = member

    with archive.open(record_name) as raw, io.TextIOWrapper(raw, encoding="utf-8", newline="") as stream:
        for row in csv.reader(stream, strict=True):
            if len(row) != 3:
                raise ValueError(f"Invalid RECORD row: {row!r}")
            name, digest, size = row
            member = files.pop(name, None)
            if member is None:
                raise ValueError(f"Duplicate or unexpected path in RECORD: {name}")
            if name == record_name:
                if digest or size:
                    raise ValueError("RECORD must have empty hash and size for itself")
                continue
            data = archive.read(member)
            expected = base64.urlsafe_b64encode(hashlib.sha256(data).digest()).rstrip(b"=").decode("ascii")
            if digest != "sha256=" + expected or size != str(len(data)):
                raise ValueError(f"Incorrect SHA-256 or size in RECORD: {name}")
    if files:
        raise ValueError(f"Files missing from RECORD: {', '.join(sorted(files))}")
