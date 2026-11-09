# Read firmware messages through the ST-LINK virtual COM port.
# Example: .\Read-Serial.ps1 -Port COM6 -DurationSeconds 30
param(
    [string]$Port = 'COM6',

    [ValidateRange(1, 2147483647)]
    [int]$BaudRate = 115200,

    [ValidateRange(1, 86400)]
    [int]$DurationSeconds = 30
)

$ErrorActionPreference = 'Stop'

# Use 8 data bits, no parity, one stop bit (8N1); no flow control.
$serial = [System.IO.Ports.SerialPort]::new(
    $Port, $BaudRate, 'None', 8, 'One'
)

try {
    $serial.Open() # Open the port; another serial reader must not own it.
    $serial.DiscardInBuffer() # Discard text already buffered on the PC.
    $until = [DateTime]::UtcNow.AddSeconds($DurationSeconds)

    while ([DateTime]::UtcNow -lt $until) {
        [Console]::Write($serial.ReadExisting()) # Display available text.
        Start-Sleep -Milliseconds 100           # Avoid continuous polling.
    }
}
finally {
    $serial.Dispose() # Release the port on completion or interruption.
}
