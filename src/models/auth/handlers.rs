pub enum ErrorGeneratingDiscriminator
{
    DatabaseError,
    TooManyUsers,
}

pub enum ErrorValidatingEmail
{
    InvalidEmail,
}

pub enum ErrorUniqueEmail
{
    DatabaseError,
    EmailNotUnique,
}

pub enum ErrorHashPassword
{
    SaltGenerationError,
}

pub enum ErrorValidatingPassword
{
    InvalidPassword,
}

pub enum ErrorGeneratingToken
{
    TokenGenerationError,
    DatabaseError,
}
