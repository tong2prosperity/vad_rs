#import <Foundation/Foundation.h>

@interface VadParams : NSObject

@property (nonatomic, assign) NSUInteger frame_size;
@property (nonatomic, assign) float threshold;
@property (nonatomic, assign) NSUInteger silence_stop_ms;
@property (nonatomic, assign) float max_speech_duration_s;
@property (nonatomic, assign) NSUInteger sample_rate;
@property (nonatomic, assign) NSUInteger pre_speech_threshold_frame_cnt;
@property (nonatomic, assign) NSUInteger speech_threshold_frame_cnt;

@end

@implementation VadParams

@end