#import <Foundation/Foundation.h>
#import "VadParams.h"

@implementation VadParams (JSONConversion)

- (NSString *)Vad2Json {
    NSDictionary *paramsDict = @{
        @"frame_size": @(self.frame_size),
        @"threshold": @(self.threshold),
        @"silence_stop_ms": @(self.silence_stop_ms),
        @"max_speech_duration_s": @(self.max_speech_duration_s),
        @"sample_rate": @(self.sample_rate),
        @"pre_speech_threshold_frame_cnt": @(self.pre_speech_threshold_frame_cnt),
        @"speech_threshold_frame_cnt": @(self.speech_threshold_frame_cnt)
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