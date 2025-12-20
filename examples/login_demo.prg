// Secure Login Demo
// Demonstrates GetSecret() for password input in a real login scenario

ClearScreen()

// Title
SetPos(0, 0)
OutStd("╔════════════════════════════════════════════════════════════════════════╗")
SetPos(1, 0)
OutStd("║                         SECURE SYSTEM ACCESS                           ║")
SetPos(2, 0)
OutStd("╚════════════════════════════════════════════════════════════════════════╗")

// Simulated database of users (in real app, this would be in a database)
local validUser := "admin"
local validPass := "secret123"

local attempts := 0
local maxAttempts := 3
local authenticated := false

while attempts < maxAttempts
    ClearScreen()

    SetPos(0, 0)
    OutStd("╔════════════════════════════════════════════════════════════════════════╗")
    SetPos(1, 0)
    OutStd("║                         SECURE SYSTEM ACCESS                           ║")
    SetPos(2, 0)
    OutStd("╚════════════════════════════════════════════════════════════════════════╝")

    SetPos(4, 5)
    OutStd("Login Attempt ")
    ? attempts + 1
    OutStd(" of ")
    ? maxAttempts

    // Input fields
    local username := Space(20)
    local password := Space(20)

    SetPos(7, 15)
    OutStd("Username: ")
    username := GetInput(username)

    SetPos(9, 15)
    OutStd("Password: ")
    password := GetSecret(password)

    // Validate credentials
    if Trim(username) == validUser
        if Trim(password) == validPass
            authenticated := true
            attempts := maxAttempts // Exit loop
        else
            attempts := attempts + 1
            if attempts < maxAttempts
                SetPos(12, 15)
                OutStd("✗ Invalid password. Please try again.")

                // Brief pause
                local delay := 0
                while delay < 50000000
                    delay := delay + 1
                enddo
            endif
        endif
    else
        attempts := attempts + 1
        if attempts < maxAttempts
            SetPos(12, 15)
            OutStd("✗ Invalid username. Please try again.")

            // Brief pause
            local delay := 0
            while delay < 50000000
                delay := delay + 1
            enddo
        endif
    endif
enddo

// Show result
ClearScreen()

if authenticated
    // Success screen
    SetPos(5, 20)
    OutStd("╔════════════════════════════════════╗")
    SetPos(6, 20)
    OutStd("║     ✓ LOGIN SUCCESSFUL             ║")
    SetPos(7, 20)
    OutStd("╚════════════════════════════════════╝")

    SetPos(10, 15)
    OutStd("Welcome back, ")
    OutStd(validUser)
    OutStd("!")
    SetPos(12, 15)
    OutStd("Access granted to secure system.")
else
    // Failure screen
    SetPos(5, 20)
    OutStd("╔════════════════════════════════════╗")
    SetPos(6, 20)
    OutStd("║     ✗ ACCESS DENIED                ║")
    SetPos(7, 20)
    OutStd("╚════════════════════════════════════╝")

    SetPos(10, 15)
    OutStd("Maximum login attempts exceeded.")
    SetPos(11, 15)
    OutStd("Account temporarily locked.")
endif

SetPos(15, 0)
OutStd("")
?
