#include <stdio.h>   		// Standard input/output definitions
#include <string.h>  		// String function definitions
#include <unistd.h>  		// UNIX standard function definitions
#include <fcntl.h>   		// File control definitions
#include <errno.h>   		// Error number definitions
#include <termios.h> 		// POSIX terminal control definitions
#include <sys/ioctl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdlib.h>



// Enable/ Dsiable RTS Pins of oure serial device
// int fd:        Serial device
// int level:     0/1 to disable/ enable RTS
int setRTS(int fd, int level) {
  int status;

  if (ioctl(fd, TIOCMGET, &status) == -1) {
    perror("setRTS(): TIOCMGET");
    return 0;
  }
  if (level)
    status |= TIOCM_RTS;
  else
    status &= ~TIOCM_RTS;
  if (ioctl(fd, TIOCMSET, &status) == -1) {
    perror("setRTS(): TIOCMSET");
    return 0;
  }
  return 1;
}



// Write binary data to Serial Device
// int fd:                  Serial device
// unsigned char data[]:    Content Bytes
// int length:              Length of the content bytes array
void serialWrite(int fd, unsigned char data[], size_t length) {
  // Disable RTS to send
  setRTS(fd, 0);
  write(fd, data, length);
  // usleep(1041*sizeof(data));
  // usleep(8328);
  usleep(8192);
}



// Read binary data from Serial Device
// int fd: Serial device
// int length: Length to read
size_t serialRead(int fd, unsigned char *arr, size_t length) {
  setRTS(fd, 1); // Enable RTS to send
  // length = (length+3); // Read 3 more bytes than requested (otherwise it did not work)

  for (unsigned int i = 0; i < length; i++) {
    // Read byte after byte
    int err = read(fd, &arr[i], 1);
    if (err == -1) return i;
  }
  return length;
}



int serialOpen(char device[]) {
  // Init HaeringAPI
  // char device[]:     Path to serial device
  struct termios options;

  int fd = open(device, O_RDWR);
  if (fd == 1) {
    fprintf(stderr, "open_port: Unable to open %s - %s\n", device, strerror(errno));
  }

  fcntl(fd, F_SETFL, FNDELAY); // Configure port reading
  tcgetattr(fd, &options);
  cfsetispeed(&options, B9600); // Set the baud rates to 19200
  cfmakeraw(&options); // Get the current options for the port

  options.c_cflag |= (CLOCAL | CREAD); // Enable the receiver and set local mode
  options.c_cflag &= ~PARENB; // Mask the character size to 8 bits, no parity
  options.c_cflag &= ~CSTOPB;
  options.c_cflag &= ~CSIZE;
  options.c_cflag |=  CS8; // Select 8 data bits
  options.c_cflag &= ~CRTSCTS; // Disable hardware flow control
  options.c_lflag &= ~(ICANON | ECHO | ISIG); // Enable data to be processed as raw input

  tcsetattr(fd, TCSANOW, &options); // Set the new options for the port

  return fd;
}



// Deinit HaeringAPI
void serialClose(int fd) {
  close(fd);
}
