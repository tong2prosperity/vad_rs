#import <Foundation/Foundation.h>
#import "VadParams.h"

@implementation VadParams (JSONConversion)

- (NSString *)Vad2Json {
    NSDictionary *paramsDict = @{
        @"frame_size": @(self.frameSize),
        @"threshold": @(self.threshold),
        @"min_silence_duration_ms": @(self.minSilenceDurationMs),
        @"speech_pad_ms": @(self.speechPadMs),
        @"min_speech_duration_ms": @(self.minSpeechDurationMs),
        @"max_speech_duration_s": @(self.maxSpeechDurationS),
        @"sample_rate": @(self.sampleRate)
    };

    NSError *error;
    NSData *jsonData = [NSJSONSerialization dataWithJSONObject:paramsDict options:0 error:&error];

    if (error) {
        NSLog(@"Error converting to JSON: %@", error.localizedDescription);
        return nil;
    }

    return [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
}

@end